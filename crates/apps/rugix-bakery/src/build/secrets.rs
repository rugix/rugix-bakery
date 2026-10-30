//! Cache-local identities for sensitive rule inputs without persisting their contents.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use hmac::{Hmac, Mac};
use reportify::ResultExt;
use sha2::Sha256;

use crate::BakeryResult;

use super::cache::artifact_hash;

/// A private key shared by executions using the same project cache.
pub struct SecretFingerprints([u8; 32]);

impl SecretFingerprints {
    /// Load or atomically create the key while the executor holds the cache lock.
    pub fn open(cache: &Path) -> BakeryResult<Self> {
        let path = cache.join("secret-fingerprint.key");
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let mut bytes = [0; 32];
                getrandom::fill(&mut bytes)
                    .whatever("unable to generate secret fingerprint key")?;
                let temporary = tempfile::NamedTempFile::new_in(cache)
                    .whatever("unable to create secret fingerprint key")?;
                temporary
                    .as_file()
                    .set_permissions(fs::Permissions::from_mode(0o600))
                    .whatever("unable to restrict secret fingerprint key")?;
                fs::write(temporary.path(), bytes)
                    .whatever("unable to write secret fingerprint key")?;
                temporary
                    .persist_noclobber(&path)
                    .whatever("unable to publish secret fingerprint key")?;
                bytes.to_vec()
            }
            Err(error) => return Err(error).whatever("unable to read secret fingerprint key"),
        };
        let key = bytes
            .try_into()
            .map_err(|_| reportify::whatever!("invalid secret fingerprint key"))?;
        Ok(Self(key))
    }

    /// Fingerprint the same filesystem properties as ordinary artifact inputs.
    pub fn fingerprint(&self, path: &Path) -> BakeryResult<String> {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&self.0).expect("HMAC accepts a key of any length");
        mac.update(b"rugix-secret-input-v1\0");
        mac.update(artifact_hash(path)?.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
}
