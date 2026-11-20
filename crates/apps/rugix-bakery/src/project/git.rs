//! Helpers for materializing Git sources.

use std::path::Path;
use std::sync::Arc;

use sha1::{Digest, Sha1};
use xscript::{read_str, run, LocalEnv, Run};

use reportify::ResultExt;

use crate::config::repositories::GitSourceConfig;
use crate::BakeryResult;

/// A globally unique id of a Git source.
///
/// The id is computed by hashing the URL of the source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceId(Arc<str>);

impl SourceId {
    /// Create a source id from a hex-encoded hash.
    pub(crate) fn from_hash(hash: &str) -> Self {
        Self(hash.into())
    }

    /// The string representation of the id.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The short string representation of the id.
    pub fn as_short_str(&self) -> &str {
        &self.as_str()[..6]
    }
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Compute the globally unique id of a Git source.
pub fn compute_git_source_id(config: &GitSourceConfig) -> SourceId {
    let mut hasher = Sha1::new();
    hasher.update(b"git");
    hasher.update(config.url.as_bytes());
    if let Some(inner_path) = &config.dir {
        hasher.update(inner_path.as_bytes());
    }
    SourceId(hex::encode(&hasher.finalize()[..]).into())
}

/// Check out the Git repository in the given directory.
///
/// The *fetch* flag indicates whether updates should be fetched from the remote.
pub fn check_out_git_source(
    config: &GitSourceConfig,
    path: &Path,
    fetch: bool,
) -> BakeryResult<()> {
    if !path.exists() {
        run!(["git", "clone", &config.url, path]).whatever("unable to clone repository")?;
    }
    let env = LocalEnv::new(path);
    if fetch {
        run!(env, ["git", "fetch", "--all"]).whatever("unable to fetch updates of repository")?;
    }
    macro_rules! rev_parse {
        ($rev:literal) => {
            read_str!(env, ["git", "rev-parse", "--verify", $rev]).whatever("unable to parse rev")
        };
    }
    let mut commit = rev_parse!("refs/remotes/origin/HEAD^{{commit}}")?;
    if let Some(tag) = &config.tag {
        commit = rev_parse!("refs/tags/{tag}^{{commit}}")?;
    }
    if let Some(branch) = &config.branch {
        commit = rev_parse!("refs/remotes/origin/{branch}^{{commit}}")?;
    }
    if let Some(rev) = &config.rev {
        commit = rev_parse!("{rev}^{{commit}}")?;
    }
    let head = rev_parse!("HEAD^{{commit}}")?;
    if head != commit {
        run!(env, ["git", "checkout", commit]).whatever("error checking out commit")?;
    }
    Ok(())
}
