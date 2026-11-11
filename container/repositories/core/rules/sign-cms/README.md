## Behavior

Sign `content` with RSA PKCS#1 v1.5 and SHA-256. The `certificate` input is a
PEM-encoded RSA X.509 certificate; the `key` secret contains its matching
PEM-encoded private key. The signature embeds the content, the signer
certificate, and a signing-time attribute. It includes no intermediate
certificates.

`signing-time` is Unix seconds encoded as a string. An empty value uses the
build source epoch. Accepted values range from `0` through `2524607999`. Fixed
inputs, key, signing time, and tooling produce identical signature bytes. Bind
private keys through `secrets` so Bakery handles them as sensitive inputs.

This rule is the default `signer` of
[`core/sign-bundle-rugix`](../sign-bundle-rugix/README.md).

## Example

For example, sign a file:

```toml
uses = "core/sign-cms"
inputs.content = { path = "metadata" }
inputs.certificate = { path = "keys/bundle.crt" }
secrets.key = { path = "keys/bundle.key" }
parameters.signing-time = "1700000000"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
