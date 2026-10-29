//! Prepare a private release filesystem without mutating cached layers.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use reportify::ResultExt;
use sidex::types::Str;

use crate::config::build_context::BuildContext;
use crate::config::rules::ArtifactKind;
use crate::config::systems::Architecture;
use crate::oven::system::{release_metadata, ReleaseInfo};
use crate::BakeryResult;

use super::{Artifact, ArtifactLocation, Planner, Task};

#[derive(Debug)]
pub struct ReleaseTask {
    pub archive: Artifact,
    architecture: Architecture,
    release: ReleaseInfo,
    system: String,
    source_date_epoch: Option<u64>,
}

impl Planner {
    /// Append release metadata preparation before image assembly.
    pub fn release(
        &mut self,
        layer: &BTreeMap<Str, Artifact>,
        architecture: Architecture,
        release: &ReleaseInfo,
        system: &str,
        source_date_epoch: Option<u64>,
    ) -> BTreeMap<Str, Artifact> {
        let task = self.plan.tasks.len();
        self.plan.tasks.push(Task::Release(ReleaseTask {
            archive: layer["archive"].clone(),
            architecture,
            release: release.clone(),
            system: system.to_owned(),
            source_date_epoch,
        }));
        let mut outputs = BTreeMap::new();
        for name in [
            "rootfs",
            "boot",
            "config",
            "artifacts",
            "bsp",
            "context",
            "layer",
        ] {
            outputs.insert(
                name.into(),
                Artifact {
                    kind: if name == "context" {
                        ArtifactKind::File
                    } else {
                        ArtifactKind::Directory
                    },
                    source: ArtifactLocation::Output {
                        task,
                        name: name.into(),
                    },
                },
            );
        }
        outputs
    }
}

impl ReleaseTask {
    pub fn execute(&self, archive: &Path, root: &Path) -> BakeryResult<BTreeMap<Str, PathBuf>> {
        let architecture = self.architecture;
        let source_date_epoch = match self.source_date_epoch {
            Some(epoch) => epoch,
            None => fs::metadata(archive)
                .whatever("unable to inspect completed layer")?
                .modified()
                .whatever("unable to inspect layer timestamp")?
                .duration_since(std::time::UNIX_EPOCH)
                .whatever("layer timestamp predates the Unix epoch")?
                .as_secs(),
        };
        let system = &self.system;
        let release = &self.release;
        crate::oven::archive::extract(archive, root)?;
        let info = release_metadata(release, system, source_date_epoch)?;
        let context = BuildContext {
            system: system.into(),
            architecture,
            source_date_epoch,
            release_id: info.release.id.clone().into(),
            release_version: info.release.version.clone().into(),
        };
        let info_path = root.join("roots/system/etc/rugix/system-build-info.json");
        fs::create_dir_all(info_path.parent().expect("system info has a parent"))
            .whatever("unable to create system info directory")?;
        fs::write(
            info_path,
            serde_json::to_vec_pretty(&info).whatever("unable to serialize system info")?,
        )
        .whatever("unable to write system info")?;
        let context_path = root.join("build-context.json");
        fs::write(
            &context_path,
            serde_json::to_vec(&context).whatever("unable to serialize build context")?,
        )
        .whatever("unable to write build context")?;
        let mut inputs = BTreeMap::new();
        for (name, relative) in [
            ("rootfs", "roots/system"),
            ("boot", "roots/boot"),
            ("config", "roots/config"),
            ("artifacts", "artifacts"),
            ("bsp", "bsp"),
        ] {
            let path = root.join(relative);
            if !path.exists() {
                fs::create_dir_all(&path).whatever("unable to create layer artifact directory")?;
                fs::File::open(&path)
                    .whatever("unable to open generated artifact directory")?
                    .set_times(fs::FileTimes::new().set_modified(
                        std::time::UNIX_EPOCH + std::time::Duration::from_secs(source_date_epoch),
                    ))
                    .whatever("unable to set generated artifact directory timestamp")?;
            }
            inputs.insert(name.into(), path);
        }
        inputs.insert("context".into(), context_path.clone());
        inputs.insert("layer".into(), root.to_owned());
        let timestamp = std::time::UNIX_EPOCH + std::time::Duration::from_secs(source_date_epoch);
        for path in [
            context_path.clone(),
            root.to_owned(),
            root.join("roots"),
            root.join("roots/system/etc/rugix/system-build-info.json"),
            root.join("roots/system/etc/rugix"),
            root.join("roots/system/etc"),
            root.join("roots/system"),
        ] {
            fs::File::open(path)
                .whatever("unable to open generated metadata")?
                .set_times(fs::FileTimes::new().set_modified(timestamp))
                .whatever("unable to set generated metadata timestamp")?;
        }

        Ok(inputs)
    }
}
