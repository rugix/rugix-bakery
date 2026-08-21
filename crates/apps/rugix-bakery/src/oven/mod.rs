//! Functionality for baking layers and images.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use clap::Args;
use layer::FrozenLayer;
use reportify::{bail, whatever, ResultExt};
use rugix_bundle::manifest::{self, BundleManifest, ChunkerAlgorithm};
use rugix_common::img_extract::extract_image_partitions;
use system::ReleaseInfo;
use tempfile::tempdir;
use tracing::info;
use url::Url;
use xscript::{run, Run};

use crate::config::images::PartitionTableType;
use crate::config::recipes::ParameterValue;
use crate::config::systems::{Architecture, SystemConfig, Target};
use crate::project::library::{LayerIdx, MixinIdx};
use crate::project::ProjectRef;
use crate::utils::caching::{download, Hasher};
use crate::BakeryResult;

pub mod customize;
pub mod layer;
pub mod system;
pub mod targets;

mod archive;

/// Parameter overrides loaded from parameter files.
///
/// Maps recipe names to their parameter key-value pairs.
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

#[derive(Debug)]
struct BakedSystemLayer {
    path: PathBuf,
    mixins: Vec<String>,
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
    source_date_epoch: u64,
    param_overrides: &ParameterOverrides,
    mixin_selection: &MixinSelection,
) -> BakeryResult<()> {
    let system_config = project
        .config()
        .get_system_config(system)
        .ok_or_else(|| whatever!("unable to find image {system}"))?;
    info!("baking image `{system}`");
    let layer_bakery = LayerBakery::new(project, system_config.architecture, param_overrides);
    let baked_layer =
        layer_bakery.bake_system_layer(system_config, mixin_selection, source_date_epoch)?;
    let frozen = FrozenLayer::new(
        frozen_layer_name(&system_config.layer, &baked_layer.mixins),
        baked_layer.path,
    );
    system::make_system(
        system_config,
        release_info,
        system,
        &frozen,
        output,
        source_date_epoch,
        &baked_layer.mixins,
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

    pub fn bake_root(&self, layer: &str, source_date_epoch: u64) -> BakeryResult<PathBuf> {
        let library = self.project.library()?;
        let Some(layer) = library.lookup_layer(library.repositories.root_repository, layer) else {
            bail!("unable to find layer {layer}");
        };
        self.bake(layer, source_date_epoch)
    }

    fn bake_system_layer(
        &self,
        system_config: &SystemConfig,
        mixin_selection: &MixinSelection,
        source_date_epoch: u64,
    ) -> BakeryResult<BakedSystemLayer> {
        let mut path = self.bake_root(&system_config.layer, source_date_epoch)?;
        let mixins = self.resolve_mixins(system_config, mixin_selection)?;
        let mut mixin_names = Vec::new();
        for mixin in mixins {
            let library = self.project.library()?;
            mixin_names.push(library.mixins[mixin].name.clone());
            drop(library);
            path = self.bake_mixin(mixin, &path, source_date_epoch)?;
        }
        Ok(BakedSystemLayer {
            path,
            mixins: mixin_names,
        })
    }

    pub fn bake(&self, layer: LayerIdx, source_date_epoch: u64) -> BakeryResult<PathBuf> {
        let repositories = &self.project.repositories()?.repositories;
        let library = self.project.library()?;
        let layer = &library.layers[layer];
        info!("baking layer `{}`", layer.name);
        let Some(config) = layer.config(self.arch) else {
            bail!("no layer configuration for architecture `{}`", self.arch);
        };
        let mut layer_id = Hasher::new();
        layer_id.push("layer", &layer.name);
        layer_id.push("repository", repositories[layer.repo].source.id.as_str());
        layer_id.push("arch", self.arch.as_str());
        push_parameter_overrides(&mut layer_id, self.param_overrides);
        if let Some(url) = &config.url {
            layer_id.push("url", url);
            let layer_id = layer_id.finalize();
            let system_tar = self
                .project
                .dir()
                .join(format!(".rugix/layers/{layer_id}/system.tar"));
            if !system_tar.exists() {
                extract(self.project, url, &system_tar)?;
            }
            Ok(system_tar)
        } else if let Some(parent) = &config.parent {
            layer_id.push("parent", parent);
            let Some(parent) = library.lookup_layer(layer.repo, parent) else {
                bail!("unable to find layer `{parent}`");
            };
            let src = self.bake(parent, source_date_epoch)?;
            let layer_id = layer_id.finalize();
            let layer_path = PathBuf::from(format!(".rugix/layers/{layer_id}"));
            let target = self.project.dir().join(&layer_path).join("system.tar");
            fs::create_dir_all(target.parent().unwrap()).ok();
            customize::customize(
                self.project,
                self.arch,
                layer,
                Some(&src),
                &target,
                &layer_path,
                source_date_epoch,
                self.param_overrides,
            )?;
            Ok(target)
        } else if config.root.unwrap_or(false) {
            layer_id.push("bare", "true");
            let layer_id = layer_id.finalize();
            let layer_path = PathBuf::from(format!(".rugix/layers/{layer_id}"));
            let target = self.project.dir().join(&layer_path).join("system.tar");
            fs::create_dir_all(target.parent().unwrap()).ok();
            customize::customize(
                self.project,
                self.arch,
                layer,
                None,
                &target,
                &layer_path,
                source_date_epoch,
                self.param_overrides,
            )?;
            Ok(target)
        } else {
            bail!("invalid layer configuration")
        }
    }

    fn bake_mixin(
        &self,
        mixin: MixinIdx,
        src: &Path,
        source_date_epoch: u64,
    ) -> BakeryResult<PathBuf> {
        let repositories = &self.project.repositories()?.repositories;
        let library = self.project.library()?;
        let mixin = &library.mixins[mixin];
        info!("applying mixin `{}`", mixin.name);
        if mixin.config(self.arch).is_none() {
            bail!("no mixin configuration for architecture `{}`", self.arch);
        }
        let mut layer_id = Hasher::new();
        layer_id.push("mixin", &mixin.name);
        layer_id.push("repository", repositories[mixin.repo].source.id.as_str());
        layer_id.push("arch", self.arch.as_str());
        layer_id.push("source", src.to_string_lossy().as_ref());
        push_parameter_overrides(&mut layer_id, self.param_overrides);
        let layer_id = layer_id.finalize();
        let layer_path = PathBuf::from(format!(".rugix/layers/{layer_id}"));
        let target = self.project.dir().join(&layer_path).join("system.tar");
        fs::create_dir_all(target.parent().unwrap()).ok();
        let applied = customize::customize_mixin(
            self.project,
            self.arch,
            mixin,
            src,
            &target,
            &layer_path,
            source_date_epoch,
            self.param_overrides,
        )?;
        if applied {
            Ok(target)
        } else {
            Ok(src.to_path_buf())
        }
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

fn push_parameter_overrides(layer_id: &mut Hasher, param_overrides: &ParameterOverrides) {
    if param_overrides.is_empty() {
        return;
    }
    let mut sorted_overrides: Vec<_> = param_overrides.iter().collect();
    sorted_overrides.sort_by_key(|(k, _)| k.as_str());
    for (recipe, params) in &sorted_overrides {
        let mut sorted_params: Vec<_> = params.iter().collect();
        sorted_params.sort_by_key(|(k, _)| k.as_str());
        for (param, value) in &sorted_params {
            layer_id.push(&format!("override:{recipe}:{param}"), value.to_string());
        }
    }
}

fn frozen_layer_name(layer: &str, mixins: &[String]) -> String {
    if mixins.is_empty() {
        layer.to_owned()
    } else {
        format!("{}+{}", layer, mixins.join("+"))
    }
}

fn extract(project: &ProjectRef, image_url: &str, layer_path: &Path) -> BakeryResult<()> {
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
    let bundle_dir = tempdir().whatever("unable to create temporary directory")?;
    let bundle_dir = bundle_dir.path();
    let system_config = project.config().resolve_system_config(system)?;
    let is_gpt = system_config
        .image
        .as_ref()
        .and_then(|img| {
            img.layout
                .as_ref()
                .map(|layout| layout.ty == Some(PartitionTableType::Gpt))
        })
        .unwrap_or(false);
    let config = match system_config.target.clone().unwrap_or(Target::Unknown) {
        Target::GenericGrubEfi => efi_bundle_config(opts),
        Target::RpiTryboot => rpi_bundle_config(opts, is_gpt),
        Target::RpiUboot => rpi_bundle_config(opts, is_gpt),
        Target::Bsp => bsp_bundle_config(system_path, opts)?,
        Target::Unknown => bail!("cannot bake bundles for unknown targets"),
    };
    std::fs::write(
        bundle_dir.join("rugix-bundle.toml"),
        toml::to_string(&config).unwrap(),
    )
    .whatever("unable to write bundle config")?;
    std::os::unix::fs::symlink(
        system_path
            .join("filesystems")
            .canonicalize()
            .whatever("unable to canonicalize filesystems directory")?,
        bundle_dir.join("payloads"),
    )
    .whatever("unable to symlink filesystems")?;
    info!("Creating bundle, this may take a while...");
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let hash =
        rugix_bundle::builder::pack(bundle_dir, output).whatever("unable to create bundle")?;
    Ok(hash)
}

fn rpi_bundle_config(opts: &BundleOpts, is_gpt: bool) -> BundleManifest {
    let compression = if opts.disable_compression {
        None
    } else {
        Some(manifest::Compression::Xz(manifest::XzCompression::new()))
    };
    manifest::BundleManifest::new(
        manifest::UpdateType::Full,
        vec![
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: "boot".to_owned(),
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
                    slot: "system".to_owned(),
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
        ],
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
        vec![
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: "boot".to_owned(),
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
                    slot: "system".to_owned(),
                }),
                "partition-4.img".to_owned(),
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression),
            )),
        ],
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
                file.clone()
            } else {
                panic!(
                    "bundle payload for slot {:?} must have either `partition` or `file`",
                    p.slot
                );
            };
            manifest::Payload::new(
                manifest::DeliveryConfig::Slot(manifest::SlotDeliveryConfig {
                    slot: p.slot.clone(),
                }),
                filename,
            )
            .with_block_encoding(Some(
                manifest::BlockEncoding::new(opts.chunker_algorithm())
                    .with_deduplicate(Some(true))
                    .with_compression(compression.clone()),
            ))
        })
        .collect();

    Ok(manifest::BundleManifest::new(
        manifest::UpdateType::Full,
        payloads,
    ))
}
