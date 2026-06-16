use std::collections::HashMap;

use crate::config::mixins::MixinConfig;
use crate::config::systems::Architecture;
use crate::utils::caching::ModificationTime;

use super::repositories::RepositoryIdx;

#[derive(Debug)]
pub struct Mixin {
    pub name: String,
    pub repo: RepositoryIdx,
    pub modified: ModificationTime,
    pub default_config: Option<MixinConfig>,
    pub arch_configs: HashMap<Architecture, MixinConfig>,
}

impl Mixin {
    pub fn new(name: String, repo: RepositoryIdx, modified: ModificationTime) -> Self {
        Self {
            name,
            repo,
            modified,
            default_config: None,
            arch_configs: HashMap::new(),
        }
    }

    /// The mixin configuration for the given architecture.
    pub fn config(&self, arch: Architecture) -> Option<&MixinConfig> {
        self.arch_configs
            .get(&arch)
            .or(self.default_config.as_ref())
    }
}
