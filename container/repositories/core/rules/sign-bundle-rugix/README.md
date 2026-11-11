## Behavior

Authenticate an existing Rugix update bundle. The rule extracts the bundle's
signed metadata, signs it with the `signer` rule parameter, verifies the
signature against `certificate`, and appends it to the bundle.

The `certificate` input contains one PEM-encoded signing certificate. The
bundle embeds only the signer certificate, without intermediates. Issue it
directly from the root that Rugix Ctrl trusts. The signer must not be a CA
certificate. Concatenated PEM certificate chains are not supported by this
rule.

`signing-time` is Unix seconds encoded as a string; an empty value uses the
build source epoch.

## Signer

The default signer, [`core/sign-cms`](../sign-cms/README.md), signs with RSA
PKCS#1 v1.5 and SHA-256 using a private key file. Bind its `key` secret through
`rules.signer`. It accepts signing times from `0` through `2524607999`. Fixed
inputs, key, signing time, and tooling produce identical signed bytes.

To sign with an HSM or a KMS, bind another rule to `signer`. It receives the
`content` and `certificate` inputs and the `signing-time` parameter. It returns
`signature`: a DER-encoded CMS SignedData with the encapsulated content and the
signer certificate, such as the output of
`openssl cms -sign -binary -nodetach -outform DER`. Bakery rejects a signature
that `certificate` does not verify or that covers other content.

Bakery verifies the signature with the checks of the current Rugix Ctrl, and
Rugix Ctrl verifies it again on the device. Rugix Ctrl 1.4 and earlier reject
the RSA signatures that `openssl cms` creates with PKCS#11 keys, which Bakery
accepts. For signers using `openssl cms` with devices on these versions, use an
ECDSA key.

## Example

For example, authenticate a prepared update bundle:

```toml
uses = "core/sign-bundle-rugix"
inputs.bundle = { path = "release/update.rugixb" }
inputs.certificate = { path = "keys/bundle.crt" }
rules.signer.secrets.key = { path = "keys/bundle.key" }
parameters.signing-time = "1700000000"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. The outputs are `bundle`
and `hash`.
