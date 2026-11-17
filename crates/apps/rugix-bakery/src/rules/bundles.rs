//! CMS signatures for Rugix update bundles.
//!
//! Signing a bundle has three steps: extract the metadata to sign, create a CMS
//! signature with a replaceable signer, and attach the signature after verifying it
//! against the signer certificate. Only the middle step needs the signing key.

use std::fs;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use reportify::{bail, ResultExt};

use super::{check_rsa_key, signing_time};
use crate::BakeryResult;

/// Sign `content` with an RSA key, embedding the content and signer certificate.
pub fn sign_cms() -> BakeryResult<()> {
    let time = signing_time()?;
    check_rsa_key()?;
    let content = fs::read("/run/rugix/bakery/work/inputs/content")
        .whatever("unable to read content to sign")?;
    let certificate = fs::read("/run/rugix/bakery/work/inputs/certificate")
        .whatever("unable to read signer certificate")?;
    let key = zeroize::Zeroizing::new(
        fs::read("/run/rugix/bakery/work/secrets/key").whatever("unable to read signer key")?,
    );
    let signer = rugix_pki::CmsSignerBuilder::new(&certificate, &key)
        .whatever("unable to create CMS signer")?
        .with_rsa_mode(rugix_pki::RsaSignatureMode::Pkcs1v15)
        .with_signing_time(UNIX_EPOCH + Duration::from_secs(time))
        .build()
        .whatever("unable to build CMS signer")?;
    let signature = signer.sign(&content).whatever("unable to sign content")?;
    fs::write("/run/rugix/bakery/work/outputs/signature.der", signature)
        .whatever("unable to write CMS signature")
}

/// Extract the metadata that a bundle signature must cover.
pub fn prepare_signature() -> BakeryResult<()> {
    let metadata = rugix_bundle::signed_metadata(Path::new("/run/rugix/bakery/work/inputs/bundle"))
        .whatever("unable to read signed bundle metadata")?;
    fs::write("/run/rugix/bakery/work/outputs/metadata", metadata)
        .whatever("unable to write signed bundle metadata")
}

/// Verify a CMS signature against the signer certificate and append it to the bundle.
///
/// Verification applies Rugix Ctrl's CMS and cryptographic checks with the
/// certificate input as signer and requires the signed content to be the bundle's
/// metadata. Devices additionally validate the certificate chain to their trusted root.
pub fn attach_signature() -> BakeryResult<()> {
    let bundle = Path::new("/run/rugix/bakery/work/inputs/bundle");
    let signature = fs::read("/run/rugix/bakery/work/inputs/signature")
        .whatever("unable to read bundle signature")?;
    let certificate = fs::read("/run/rugix/bakery/work/inputs/certificate")
        .whatever("unable to read signer certificate")?;
    let signed = rugix_pki::verify_cms_with_certificate(&signature, &certificate)
        .whatever("bundle signature does not verify with the signer certificate")?;
    let metadata =
        rugix_bundle::signed_metadata(bundle).whatever("unable to read signed bundle metadata")?;
    if signed != metadata {
        bail!("bundle signature covers content other than the bundle metadata");
    }
    rugix_bundle::add_bundle_signature(
        bundle,
        signature,
        Path::new("/run/rugix/bakery/work/outputs/update.rugixb"),
    )
    .whatever("unable to append bundle signature")?;
    super::write_bundle_hash()
}
