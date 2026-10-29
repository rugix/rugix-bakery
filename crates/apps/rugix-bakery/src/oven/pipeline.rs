//! Translate familiar system configuration into ordinary core rule invocations.
//!
//! BSP layouts and partition capacities can depend on layer outputs. Deferred
//! expansion reads those artifacts and adds nodes to the same execution session.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use reportify::{bail, whatever, ResultExt};
use rugix_common::disk::PartitionTable;
use sidex::types::Str;

use crate::build::{Artifact, ArtifactLocation, Plan, Planner};
use crate::config::images::{Filesystem, ImageLayout};
use crate::config::recipes::ParameterValue;
use crate::config::rules::ArtifactKind;
use crate::config::systems::{SystemConfig, Target};
use crate::project::repositories::ProjectRepositories;
use crate::BakeryResult;

use super::{system, targets, BundleOpts};

#[derive(Clone)]
struct Pipeline {
    repositories: Arc<ProjectRepositories>,
    config: SystemConfig,
    epoch: u64,
    bundle: Option<BundleOpts>,
}

/// Append the predefined pipeline without executing layers outside the graph.
pub fn plan(
    planner: &mut Planner,
    inputs: BTreeMap<Str, Artifact>,
    config: &SystemConfig,
    bundle: Option<&BundleOpts>,
) -> BTreeMap<Str, Artifact> {
    let pipeline = Pipeline {
        repositories: planner.repositories.clone(),
        config: config.clone(),
        epoch: 0,
        bundle: bundle.cloned(),
    };
    let outputs = pipeline.output_kinds();
    planner.defer(inputs, outputs, move |inputs| {
        pipeline.resolve_layout(inputs)
    })
}

impl Pipeline {
    fn output_kinds(&self) -> BTreeMap<Str, ArtifactKind> {
        let mut outputs = BTreeMap::from([
            ("image".into(), ArtifactKind::File),
            ("filesystems".into(), ArtifactKind::Directory),
            ("artifacts".into(), ArtifactKind::Directory),
            ("bsp".into(), ArtifactKind::Directory),
            ("system-build-info.json".into(), ArtifactKind::File),
            ("sbom.spdx.json".into(), ArtifactKind::File),
            ("sbom.cdx.json".into(), ArtifactKind::File),
        ]);
        if self.bundle.is_some() {
            outputs.insert("bundle".into(), ArtifactKind::File);
        }
        outputs
    }

    fn resolve_layout(&self, paths: &BTreeMap<Str, PathBuf>) -> BakeryResult<Plan> {
        let mut planner = Planner::new(self.repositories.clone())?;
        let layout = if let Some(layout) = self
            .config
            .image
            .as_ref()
            .and_then(|image| image.layout.clone())
        {
            layout
        } else if matches!(self.config.target, Some(Target::Bsp)) {
            system::bsp_to_image_layout(&system::load_bsp_config(
                &paths["bsp"].join("rugix-bsp.toml"),
            )?)
        } else {
            self.config
                .target
                .as_ref()
                .and_then(|target| {
                    targets::get_default_layout(
                        target,
                        self.config
                            .options
                            .as_ref()
                            .and_then(|options| options.use_squashfs.as_ref()),
                    )
                })
                .ok_or_else(|| whatever!("image layout needs to be specified"))?
        };
        let context: crate::config::build_context::BuildContext =
            crate::config::load_json(&paths["context"])?;
        let mut pipeline = self.clone();
        pipeline.epoch = context.source_date_epoch;
        let mut inputs = path_artifacts(paths)?;
        let initialized = planner.core_rule(
            "initialize-boot",
            BTreeMap::from([
                ("boot".into(), inputs["boot"].clone()),
                ("config".into(), inputs["config"].clone()),
            ]),
            BTreeMap::from([
                (
                    "target".into(),
                    ParameterValue::String(
                        target_name(self.config.target.as_ref().unwrap_or(&Target::Unknown)).into(),
                    ),
                ),
                (
                    "architecture".into(),
                    ParameterValue::String(self.config.architecture.as_str().into()),
                ),
                ("source-date-epoch".into(), integer(pipeline.epoch)?),
            ]),
        )?;
        inputs.insert("boot".into(), initialized["boot"].clone());
        inputs.insert("config".into(), initialized["config"].clone());
        let mut roots = BTreeMap::new();
        for root in layout
            .partitions
            .iter()
            .flatten()
            .filter_map(|partition| partition.root.as_deref())
        {
            validate_root(root)?;
            let first = Path::new(root)
                .components()
                .next()
                .expect("root was validated")
                .as_os_str()
                .to_str()
                .expect("configuration is UTF-8");
            roots.insert(
                PathBuf::from(first),
                match first {
                    "boot" => inputs["boot"].clone(),
                    "config" => inputs["config"].clone(),
                    _ => select(
                        inputs["layer"].clone(),
                        &format!("roots/{first}"),
                        ArtifactKind::Directory,
                    )?,
                },
            );
        }
        let disk = planner.core_rule(
            "image-layout",
            BTreeMap::from([
                ("layout".into(), Artifact::json(&layout)?),
                ("layer".into(), inputs["layer"].clone()),
                ("roots".into(), directory(roots)),
            ]),
            BTreeMap::from([(
                "size".into(),
                integer(
                    self.config
                        .image
                        .as_ref()
                        .and_then(|image| image.size)
                        .map_or(0, |size| size.raw),
                )?,
            )]),
        )?;
        inputs.insert("image".into(), disk["image"].clone());
        let outputs = planner.defer(inputs, self.output_kinds(), move |inputs| {
            pipeline.populate(inputs, &layout)
        });
        Ok(planner.finish(outputs))
    }

    fn populate(&self, paths: &BTreeMap<Str, PathBuf>, layout: &ImageLayout) -> BakeryResult<Plan> {
        let mut planner = Planner::new(self.repositories.clone())?;
        let inputs = path_artifacts(paths)?;
        let table = PartitionTable::read(&paths["image"])
            .whatever("unable to read resolved disk geometry")?;
        let boot = planner.core_rule(
            "prepare-boot",
            BTreeMap::from([
                ("boot".into(), inputs["boot"].clone()),
                ("image".into(), inputs["image"].clone()),
            ]),
            BTreeMap::from([
                (
                    "target".into(),
                    ParameterValue::String(
                        target_name(self.config.target.as_ref().unwrap_or(&Target::Unknown)).into(),
                    ),
                ),
                ("source-date-epoch".into(), integer(self.epoch)?),
            ]),
        )?;
        let mut filesystems = BTreeMap::new();
        let mut partition_images = BTreeMap::new();
        for (specification, partition) in layout.partitions.iter().flatten().zip(&table.partitions)
        {
            let Some(filesystem) = &specification.filesystem else {
                continue;
            };
            let tree = match specification.root.as_deref() {
                Some(root) => root_tree(&inputs, &boot, root)?,
                None => directory(BTreeMap::new()),
            };
            let mut parameters =
                BTreeMap::from([("source-date-epoch".into(), integer(self.epoch)?)]);
            let capacity = table.blocks_to_bytes(partition.size).into_raw();
            let name = match filesystem {
                Filesystem::Ext4(options) => {
                    parameters.insert("size".into(), integer(capacity)?);
                    parameters.insert(
                        "mkfs-options".into(),
                        shell_options(options.additional_options.as_deref())?,
                    );
                    let archive_epoch = options
                        .clamp_mtime
                        .map_or(self.epoch as i64, |time| time.unix_seconds());
                    let archive = planner.core_rule(
                        "archive-tree",
                        BTreeMap::from([("tree".into(), tree.clone())]),
                        BTreeMap::from([(
                            "source-date-epoch".into(),
                            ParameterValue::Integer(archive_epoch),
                        )]),
                    )?;
                    filesystems.insert(
                        PathBuf::from(format!("partition-{}.tar", partition.number)),
                        archive["archive"].clone(),
                    );
                    "image-ext4"
                }
                Filesystem::Fat32 => {
                    parameters.insert("size".into(), integer(capacity)?);
                    "image-fat"
                }
                Filesystem::Squashfs(options) => {
                    parameters.insert("compression".into(), ParameterValue::String("gzip".into()));
                    let mut args = options
                        .additional_options
                        .as_deref()
                        .unwrap_or_default()
                        .to_vec();
                    if options.no_compression.unwrap_or(false) {
                        args.extend(["-noI", "-noD", "-noF", "-noX"].map(Str::from));
                    }
                    parameters.insert("mkfs-options".into(), shell_options(Some(&args))?);
                    "image-squashfs"
                }
            };
            let result =
                planner.core_rule(name, BTreeMap::from([("tree".into(), tree)]), parameters)?;
            filesystems.insert(
                PathBuf::from(format!("partition-{}.img", partition.number)),
                result["image"].clone(),
            );
            partition_images.insert(
                PathBuf::from(partition.number.to_string()),
                result["image"].clone(),
            );
        }
        let image = planner.core_rule(
            "assemble-disk",
            BTreeMap::from([
                ("image".into(), inputs["image"].clone()),
                ("partitions".into(), directory(partition_images)),
            ]),
            BTreeMap::new(),
        )?;
        let sbom = planner.core_rule(
            "sbom",
            BTreeMap::from([
                ("tree".into(), inputs["rootfs"].clone()),
                ("context".into(), inputs["context"].clone()),
            ]),
            BTreeMap::new(),
        )?;
        let filesystem_tree = directory(filesystems.clone());
        let mut outputs = BTreeMap::from([
            ("image".into(), image["image"].clone()),
            ("filesystems".into(), filesystem_tree.clone()),
            ("artifacts".into(), inputs["artifacts"].clone()),
            ("bsp".into(), inputs["bsp"].clone()),
            (
                "system-build-info.json".into(),
                select(
                    inputs["rootfs"].clone(),
                    "etc/rugix/system-build-info.json",
                    ArtifactKind::File,
                )?,
            ),
            ("sbom.spdx.json".into(), sbom["spdx"].clone()),
            ("sbom.cdx.json".into(), sbom["cyclonedx"].clone()),
        ]);
        if let Some(options) = &self.bundle {
            let target = self.config.target.as_ref().unwrap_or(&Target::Unknown);
            let manifest = match target {
                Target::GenericGrubEfi => super::efi_bundle_config(options),
                Target::RpiTryboot | Target::RpiUboot => {
                    super::rpi_bundle_config(options, table.is_gpt())
                }
                Target::Bsp => super::bsp_bundle_config(paths["layer"].as_path(), options)?,
                Target::Unknown => bail!("cannot bake bundles for unknown targets"),
            };
            let mut payloads = filesystems;
            if matches!(target, Target::Bsp) {
                let bsp = system::load_bsp_config(&paths["bsp"].join("rugix-bsp.toml"))?;
                for payload in bsp.bundle.iter().flat_map(|bundle| &bundle.payloads) {
                    if let Some(file) = &payload.file {
                        if payload.partition.is_some() {
                            bail!("BSP payload must select either a partition or a file");
                        }
                        let source = select(inputs["artifacts"].clone(), file, ArtifactKind::File)?;
                        if payloads
                            .insert(PathBuf::from(file.as_str()), source)
                            .is_some()
                        {
                            bail!("BSP file payload collides with a partition artifact");
                        }
                    }
                }
            }
            let bundle = planner.core_rule(
                "bundle-rugix",
                BTreeMap::from([
                    (
                        "manifest".into(),
                        Artifact {
                            kind: ArtifactKind::File,
                            source: ArtifactLocation::Inline(
                                toml::to_string(&manifest)
                                    .whatever("unable to encode bundle manifest")?
                                    .into_bytes(),
                            ),
                        },
                    ),
                    ("payloads".into(), directory(payloads)),
                ]),
                BTreeMap::new(),
            )?;
            outputs.insert("bundle".into(), bundle["bundle"].clone());
        }
        Ok(planner.finish(outputs))
    }
}

/// Nested boot/config roots must use the same prepared trees as layout sizing.
fn root_tree(
    inputs: &BTreeMap<Str, Artifact>,
    boot: &BTreeMap<Str, Artifact>,
    root: &str,
) -> BakeryResult<Artifact> {
    let path = Path::new(root);
    for (name, source) in [("boot", &boot["boot"]), ("config", &inputs["config"])] {
        if let Ok(relative) = path.strip_prefix(name) {
            return if relative.as_os_str().is_empty() {
                Ok(source.clone())
            } else {
                select(
                    source.clone(),
                    relative.to_str().expect("configuration paths are UTF-8"),
                    ArtifactKind::Directory,
                )
            };
        }
    }
    select(
        inputs["layer"].clone(),
        &format!("roots/{root}"),
        ArtifactKind::Directory,
    )
}

fn path_artifacts(paths: &BTreeMap<Str, PathBuf>) -> BakeryResult<BTreeMap<Str, Artifact>> {
    paths
        .iter()
        .map(|(name, path)| Ok((name.clone(), Artifact::from_path(path)?)))
        .collect()
}

fn directory(entries: BTreeMap<PathBuf, Artifact>) -> Artifact {
    Artifact {
        kind: ArtifactKind::Directory,
        source: ArtifactLocation::Tree(entries),
    }
}

fn select(source: Artifact, path: &str, kind: ArtifactKind) -> BakeryResult<Artifact> {
    let path = Path::new(path);
    if path
        .components()
        .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        bail!("filesystem root must be a relative path without '..'");
    }
    Ok(Artifact {
        kind,
        source: ArtifactLocation::Select {
            source: Box::new(source),
            path: path.into(),
        },
    })
}

fn integer(value: u64) -> BakeryResult<ParameterValue> {
    Ok(ParameterValue::Integer(
        i64::try_from(value).whatever("rule parameter exceeds integer range")?,
    ))
}

fn shell_options(options: Option<&[Str]>) -> BakeryResult<ParameterValue> {
    Ok(ParameterValue::String(
        shlex::try_join(
            options
                .unwrap_or_default()
                .iter()
                .map(|value| value.as_str()),
        )
        .whatever("formatter arguments contain NUL")?
        .into(),
    ))
}

fn validate_root(root: &str) -> BakeryResult<()> {
    if root.is_empty()
        || Path::new(root)
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        bail!("filesystem root must be a nonempty relative path without '..'");
    }
    Ok(())
}

fn target_name(target: &Target) -> &'static str {
    match target {
        Target::RpiTryboot => "rpi-tryboot",
        Target::RpiUboot => "rpi-uboot",
        Target::GenericGrubEfi => "generic-grub-efi",
        Target::Bsp => "bsp",
        Target::Unknown => "unknown",
    }
}
