## Behavior

Authenticate an existing Rugix update bundle. The `certificate` input contains
one PEM-encoded RSA signing certificate; the `key` secret contains its matching
PEM-encoded private key. The bundle embeds only the signer certificate, without
intermediates. Issue it directly from the root that Rugix Ctrl trusts. The
signer must not be a CA certificate. Concatenated PEM certificate chains are
not supported by this rule.

The signature uses RSA PKCS#1 v1.5 with SHA-256. `signing-time` is Unix seconds
encoded as a string; an empty value uses the build source epoch. Accepted values
range from `0` through `2524607999`. Fixed inputs, key, signing time, and
tooling produce identical signed bytes. Bind private keys through `secrets`
so Bakery handles them as sensitive inputs.

## Example

For example, authenticate a prepared update bundle:

```toml
uses = "core/sign-bundle-rugix"
inputs.bundle = { path = "release/update.rugixb" }
inputs.certificate = { path = "keys/bundle.crt" }
secrets.key = { path = "keys/bundle.key" }
parameters.signing-time = "1700000000"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. The output is `bundle`.


The `hash` output contains the bundle identity used to verify updates.
