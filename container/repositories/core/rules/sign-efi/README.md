## Behavior

Sign a PE executable, such as a UKI or bootloader, with RSA and SHA-256 using
`osslsigncode`. The `certificate` input is a PEM-encoded RSA X.509 certificate;
the `key` secret contains its matching PEM-encoded private key. The rule verifies
the resulting signature before exporting `image`.

`signing-time` is Unix seconds encoded as a string. An empty value uses the
build source epoch. EFI signing accepts values from `0` through `2524607999`.
Fixed inputs, key, signing time, and tooling produce identical signed bytes.
Bind private keys through `secrets` so Bakery handles them as sensitive inputs.

The device's firmware must trust the signing certificate. Certificate enrollment
and firmware trust configuration are separate provisioning steps.

This rule is the default `efi-signer` of
[`core/image-efi-verified`](../image-efi-verified/README.md#signers), which
accepts other signers, for example HSM-backed ones.

## Example

For example, sign a prepared UKI:

```toml
uses = "core/sign-efi"
inputs.image = { path = "boot.efi" }
inputs.certificate = { path = "keys/boot.crt" }
secrets.key = { path = "keys/boot.key" }
parameters.signing-time = "1700000000"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
