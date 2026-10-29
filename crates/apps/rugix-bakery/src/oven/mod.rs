//! Functionality for baking layers and images.

use crate::build::{Artifact, Executor, Planner};
use sidex::types::Str;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use clap::Args;
use reportify::{bail, whatever, ResultExt};
use rugix_bundle::manifest::{self, BundleManifest, ChunkerAlgorithm};
use rugix_common::img_extract::extract_image_partitions;
use system::ReleaseInfo;
use tempfile::tempdir;
use tracing::info;
use url::Url;
use xscript::{run, Run};

use crate::config::recipes::ParameterValue;
use crate::config::systems::{Architecture, SystemConfig};
use crate::project::library::{LayerIdx, Library, MixinIdx};
use crate::project::ProjectRef;
use crate::utils::caching::{download, Hasher};
use crate::BakeryResult;

mod build;
pub mod customize;
pub mod layer;
mod pipeline;
pub mod system;
pub mod targets;

pub(crate) mod archive;

/// Parameter overrides loaded from parameter files.
///
/// Maps recipe references, resolved from the project repository, to their parameter
/// key-value pairs. For example, `core/ssh` selects the core recipe and `ssh` selects
/// a project recipe.
pub type ParameterOverrides = HashMap<String, HashMap<String, ParameterValue>>;

/// Build-time mixin selection.
#[derive(Clone, Debug, Default)]
pub struct MixinSelection {
    /// Mixins to enable for this bake in addition to system defaults.
    pub enable: Vec<String>,
    /// Mixins to disable for this bake.
    pub disable: Vec<String>,
    /// Do not enable mixins configured on the system by default.
    pub no_default_mixins: bool,
}

/// Load and merge parameter overrides from the given parameter files.
///
/// Later files take precedence over earlier ones.
pub fn load_parameter_overrides(param_files: &[PathBuf]) -> BakeryResult<ParameterOverrides> {
    let mut overrides = ParameterOverrides::new();
    for path in param_files {
        let contents = fs::read_to_string(path)
            .whatever_with(|_| format!("unable to read parameter file {path:?}"))?;
        let file_overrides: ParameterOverrides = toml::from_str(&contents)
            .whatever_with(|_| format!("unable to parse parameter file {path:?}"))?;
        for (recipe, params) in file_overrides {
            overrides.entry(recipe).or_default().extend(params);
        }
    }
    Ok(overrides)
}

pub fn bake_system(
    project: &ProjectRef,
    release_info: &ReleaseInfo,
    system: &str,
    output: &Path,
    source_date_epoch: Option<u64>,
    param_overrides: &ParameterOverrides,
    mixin_selection: &MixinSelection,
    bundle_options: Option<&BundleOpts>,
) -> BakeryResult<()> {
    let system_config = project
        .config()
        .get_system_config(system)
        .ok_or_else(|| whatever!("unable to find image {system}"))?;
    if system_config.build.is_some()
        && (system_config.target.is_some()
            || system_config.image.is_some()
            || system_config.options.is_some())
    {
        bail!("repository-defined builds cannot be combined with target, image, or options; configure the assembly rule instead");
    }
    info!("baking image `{system}`");
    let layer_bakery = LayerBakery::new(project, system_config.architecture, param_overrides);
    let mut planner = Planner::new(project.repositories()?)?.with_layers(
        project,
        source_date_epoch,
        param_overrides,
    );
    let baked_layer =
        layer_bakery.plan_system_layer(&mut planner, system_config, mixin_selection)?;
    build::assemble(
        project,
        planner,
        baked_layer,
        system_config,
        bundle_options,
        release_info,
        system,
        output,
        source_date_epoch,
    )
}

pub struct LayerBakery<'p> {
    project: &'p ProjectRef,
    arch: Architecture,
    param_overrides: &'p ParameterOverrides,
}

impl<'p> LayerBakery<'p> {
    pub fn new(
        project: &'p ProjectRef,
        arch: Architecture,
        param_overrides: &'p ParameterOverrides,
    ) -> Self {
        Self {
            project,
            arch,
            param_overrides,
        }
    }

    pub fn bake_root(&self, layer: &str, source_date_epoch: Option<u64>) -> BakeryResult<PathBuf> {
        let library = self.project.library()?;
        let index = library
            .lookup_layer(library.repositories.root_repository, layer)
            .ok_or_else(|| whatever!("unable to find layer {layer:?}"))?;
        self.bake(index, source_date_epoch)
    }

    fn plan_system_layer(
        &self,
        planner: &mut Planner,
        system_config: &SystemConfig,
        mixin_selection: &MixinSelection,
    ) -> BakeryResult<BTreeMap<Str, Artifact>> {
        let repository = self.project.repositories()?.root_repository;
        let mut outputs = planner.layer(
            repository,
            &system_config.layer,
            self.arch,
            BTreeMap::new(),
            BTreeMap::new(),
        )?;
        for mixin in self.resolve_mixins(system_config, mixin_selection)? {
            outputs = planner.mixin(mixin, self.arch, outputs)?;
        }
        Ok(outputs)
    }

    pub fn bake(&self, layer: LayerIdx, source_date_epoch: Option<u64>) -> BakeryResult<PathBuf> {
        let library = self.project.library()?;
        let layer = &library.layers[layer];
        let mut planner = Planner::new(self.project.repositories()?)?.with_layers(
            self.project,
            source_date_epoch,
            self.param_overrides,
        );
        let outputs = planner.layer(
            layer.repo,
            &layer.name,
            self.arch,
            BTreeMap::new(),
            BTreeMap::new(),
        )?;
        let plan = planner.finish(BTreeMap::from([(
            "archive".into(),
            outputs["archive"].clone(),
        )]));
        let mut executor = Executor::for_project(self.project)?;
        let archive = executor.execute(&plan)?["archive"].clone();
        executor.keep_artifact(&archive);
        Ok(archive)
    }

    fn resolve_mixins(
        &self,
        system_config: &SystemConfig,
        selection: &MixinSelection,
    ) -> BakeryResult<Vec<MixinIdx>> {
        let library = self.project.library()?;
        let root_repo = library.repositories.root_repository;
        let disabled = selection
            .disable
            .iter()
            .map(|name| {
                library
                    .lookup_mixin(root_repo, name)
                    .ok_or_else(|| whatever!("unable to find mixin `{name}`"))
            })
            .collect::<BakeryResult<HashSet<_>>>()?;

        let mut requested = Vec::new();
        if !selection.no_default_mixins {
            for name in system_config.mixins.as_deref().unwrap_or_default() {
                let idx = library
                    .lookup_mixin(root_repo, name)
                    .ok_or_else(|| whatever!("unable to find mixin `{name}`"))?;
                if !disabled.contains(&idx) {
                    requested.push(idx);
                }
            }
        }
        for name in &selection.enable {
            let idx = library
                .lookup_mixin(root_repo, name)
                .ok_or_else(|| whatever!("unable to find mixin `{name}`"))?;
            if disabled.contains(&idx) {
                bail!("mixin `{name}` is both enabled and disabled");
            }
            requested.push(idx);
        }

        let mut resolved = Vec::new();
        let mut state = HashMap::new();
        for mixin in requested {
            self.resolve_mixin(mixin, &disabled, &mut state, &mut resolved)?;
        }
        Ok(resolved)
    }

    fn resolve_mixin(
        &self,
        mixin: MixinIdx,
        disabled: &HashSet<MixinIdx>,
        state: &mut HashMap<MixinIdx, VisitState>,
        resolved: &mut Vec<MixinIdx>,
    ) -> BakeryResult<()> {
        match state.get(&mixin) {
            Some(VisitState::Done) => return Ok(()),
            Some(VisitState::Visiting) => bail!("cycle while resolving mixins"),
            None => {}
        }
        if disabled.contains(&mixin) {
            let library = self.project.library()?;
            bail!("mixin `{}` is disabled", library.mixins[mixin].name);
        }
        state.insert(mixin, VisitState::Visiting);
        let dependencies = {
            let library = self.project.library()?;
            let mixin_ref = &library.mixins[mixin];
            let mixin_config = mixin_ref.config(self.arch).ok_or_else(|| {
                whatever!("no mixin configuration for architecture `{}`", self.arch)
            })?;
            mixin_config
                .dependencies
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|dependency| {
                    let dependency_idx = library
                        .lookup_mixin(mixin_ref.repo, dependency)
                        .ok_or_else(|| whatever!("unable to find mixin `{dependency}`"))?;
                    if disabled.contains(&dependency_idx) {
                        bail!(
                            "mixin `{dependency}` is disabled but required by `{}`",
                            mixin_ref.name
                        );
                    }
                    Ok(dependency_idx)
                })
                .collect::<BakeryResult<Vec<_>>>()?
        };
        for dependency_idx in dependencies {
            self.resolve_mixin(dependency_idx, disabled, state, resolved)?;
        }
        state.insert(mixin, VisitState::Done);
        resolved.push(mixin);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VisitState {
    Visiting,
    Done,
}

pub(crate) fn push_parameter_overrides(
    layer_id: &mut Hasher,
    param_overrides: &ParameterOverrides,
    library: &Library,
) {
    if param_overrides.is_empty() {
        return;
    }
    // Invalidate layers built when overrides were matched by bare recipe name.
    layer_id.push("parameter-override-resolution", "project-repository");
    let mut sorted_overrides: Vec<_> = param_overrides.iter().collect();
    sorted_overrides.sort_by_key(|(k, _)| k.as_str());
    for (recipe, params) in &sorted_overrides {
        if let Some(idx) = library.lookup(library.repositories.root_repository, recipe) {
            let repository = library.recipes[idx].repository;
            layer_id.push(
                &format!("override-repository:{recipe}"),
                library.repositories[repository].source.id.as_str(),
            );
        }
        let mut sorted_params: Vec<_> = params.iter().collect();
        sorted_params.sort_by_key(|(k, _)| k.as_str());
        for (param, value) in &sorted_params {
            layer_id.push(&format!("override:{recipe}:{param}"), value.to_string());
        }
    }
}

pub(crate) fn extract(
    project: &ProjectRef,
    image_url: &str,
    layer_path: &Path,
) -> BakeryResult<()> {
    let image_url = image_url
        .parse::<Url>()
        .whatever("unable to parse image URL")?;
    let mut image_path = match image_url.scheme() {
        "file" => {
            let mut image_path = project.dir().to_path_buf();
            image_path.push(image_url.path().strip_prefix('/').unwrap());
            image_path
        }
        _ => download(&image_url)?,
    };
    if image_path.extension() == Some("xz".as_ref()) {
        let decompressed_image_path = image_path.with_extension("");
        if !decompressed_image_path.is_file() {
            info!("decompressing XZ image");
            run!(["xz", "-d", "-k", image_path]).whatever("unable to decompress image")?;
        }
        image_path = decompressed_image_path;
    }
    if image_path.extension() == Some("gz".as_ref()) {
        let decompressed_image_path = image_path.with_extension("");
        if !decompressed_image_path.is_file() {
            info!("decompressing GZ image");
            run!(["gzip", "-d", "-k", image_path]).whatever("unable to decompress image")?;
        }
        image_path = decompressed_image_path;
    }
    if let Some(parent) = layer_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).whatever("unable to create layer path")?;
        }
    }
    let temp_dir = tempdir().whatever("unable to create temporary directory")?;
    let temp_dir_path = temp_dir.path();
    let system_dir = temp_dir_path.join("roots/system");
    let boot_dir = temp_dir_path.join("roots/boot");
    std::fs::create_dir_all(&system_dir).whatever("unable to create system directory")?;
    std::fs::create_dir_all(&boot_dir).whatever("unable to create boot directory")?;
    if image_path.extension() == Some("tar".as_ref()) {
        info!("Copying root filesystem {image_path:?}");
        archive::extract(&image_path, &system_dir)
            .whatever("failed to extract root file system")?;
        archive::create(temp_dir_path, layer_path, None)
            .whatever("failed to create layer tar file")?;
    } else {
        info!("extracting partitions from disk image");
        extract_image_partitions(
            &image_path,
            &[(1, boot_dir.as_path()), (2, system_dir.as_path())],
            temp_dir_path,
        )
        .whatever("unable to extract partitions from disk image")?;
        archive::create(temp_dir_path, layer_path, None)
            .whatever("failed to create layer tar file")?;
    }
    Ok(())
}

/// Bundle options.
#[derive(Args, Clone, Debug)]
pub struct BundleOpts {
    /// Disable compression of the bundle.
    #[clap(long)]
    disable_compression: bool,
    /// Use a specific chunking algorithm.
    #[clap(long)]
    chunker: Option<ChunkerAlgorithm>,
}

impl BundleOpts {
    pub fn chunker_algorithm(&self) -> ChunkerAlgorithm {
        self.chunker.clone().unwrap_or(ChunkerAlgorithm::Casync {
            avg_block_size_kib: 64,
        })
    }
}

pub fn bake_bundle(
    project: &ProjectRef,
    system: &str,
    system_path: &Path,
    output: &Path,
    opts: &BundleOpts,
) -> BakeryResult<si_crypto_hashes::HashDigest> {
    let system_config = project.config().resolve_system_config(system)?;
    if system_config.build.is_some() && (opts.disable_compression || opts.chunker.is_some()) {
        bail!("configure bundle encoding through the repository-defined assembly rule");
    }
    let source = system_path.join("system.rugixb");
    if !source.is_file() {
        bail!("system assembly did not export a bundle artifact");
    }
    crate::build::copy_file(&source, output)?;
    rugix_bundle::bundle_hash(output).whatever("unable to hash assembled update bundle")
}

fn rpi_bundle_config(opts: &BundleOpts, is_gpt: bool) -> BundleManifest {
    let compression = if opts.disable_compression {
        None
    } else {
        Some(manifest::Compression::Xz(manifest::XzCompression::new()))
    };
    manifest::BundleManifest::new(
        manifest::UpdateType::Full,
        (vec![
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: "boot".to_owned().into(),
                }),
                "partition-2.img".to_owned(),
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression.clone()),
            )),
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: "system".to_owned().into(),
                }),
                if is_gpt {
                    "partition-4.img".to_owned()
                } else {
                    "partition-5.img".to_owned()
                },
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression.clone()),
            )),
        ])
        .into(),
    )
}

fn efi_bundle_config(opts: &BundleOpts) -> BundleManifest {
    let compression = if opts.disable_compression {
        None
    } else {
        Some(manifest::Compression::Xz(manifest::XzCompression::new()))
    };
    manifest::BundleManifest::new(
        manifest::UpdateType::Full,
        (vec![
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: "boot".to_owned().into(),
                }),
                "partition-2.img".to_owned(),
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression.clone()),
            )),
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: "system".to_owned().into(),
                }),
                "partition-4.img".to_owned(),
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression),
            )),
        ])
        .into(),
    )
}

fn bsp_bundle_config(system_path: &Path, opts: &BundleOpts) -> BakeryResult<BundleManifest> {
    use crate::config::bsp::BspConfig;

    let bsp_toml_path = system_path.join("bsp/rugix-bsp.toml");
    let content = fs::read_to_string(&bsp_toml_path)
        .whatever("unable to read rugix-bsp.toml for bundle config")?;
    let bsp: BspConfig = toml::from_str(&content).whatever("unable to parse rugix-bsp.toml")?;

    let bundle = bsp.bundle.as_ref();
    let payloads_list = bundle.map(|b| &b.payloads).filter(|p| !p.is_empty());
    let Some(payloads_list) = payloads_list else {
        bail!("rugix-bsp.toml has no [[bundle.payloads]] entries");
    };

    let compression = if opts.disable_compression {
        None
    } else {
        Some(manifest::Compression::Xz(manifest::XzCompression::new()))
    };

    let payloads = payloads_list
        .iter()
        .map(|p| {
            let filename = if let Some(partition) = p.partition {
                format!("partition-{partition}.img")
            } else if let Some(file) = &p.file {
                file.to_string()
            } else {
                return Err(whatever!(
                    "bundle payload for slot {:?} must have either partition or file",
                    p.slot
                ));
            };
            Ok(manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: p.slot.clone(),
                }),
                filename,
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression.clone()),
            )))
        })
        .collect::<BakeryResult<_>>()?;

    Ok(manifest::BundleManifest::new(
        manifest::UpdateType::Full,
        payloads,
    ))
}
