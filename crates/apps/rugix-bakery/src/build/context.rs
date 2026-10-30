//! Resolve reproducible execution settings once for a build session.

use reportify::{bail, ResultExt};

use crate::BakeryResult;

/// Settings inherited by every node, including nested and deferred graphs.
#[derive(Clone, Copy, Debug)]
pub struct ExecutionContext {
    pub source_date_epoch: u64,
}

impl ExecutionContext {
    /// Explicit source dates override the environment; the deterministic default is zero.
    pub fn resolve(explicit: Option<u64>) -> BakeryResult<Self> {
        let source_date_epoch = match explicit {
            Some(epoch) => epoch,
            None => match std::env::var("SOURCE_DATE_EPOCH") {
                Ok(value) => {
                    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                        bail!("SOURCE_DATE_EPOCH must be a nonnegative integer");
                    }
                    value
                        .parse::<u64>()
                        .whatever("SOURCE_DATE_EPOCH is out of range")?
                }
                Err(std::env::VarError::NotPresent) => 0,
                Err(error) => return Err(error).whatever("invalid SOURCE_DATE_EPOCH"),
            },
        };
        if source_date_epoch > i64::MAX as u64 {
            bail!("source date is out of range");
        }
        Ok(Self { source_date_epoch })
    }
}
