## Behavior

Check a CMS signature independently of the signer that created it, then append
it to the bundle. The rule applies the CMS and cryptographic checks of Rugix
Ctrl with `certificate` as the signer certificate and requires the encapsulated
content to equal the bundle's signed metadata. It rejects CA certificates and
certificates without code-signing usage, like Rugix Ctrl. It does not validate
certificate validity or a certificate chain; Rugix Ctrl validates the chain to
its trusted root when installing an update.

The `hash` output contains the bundle hash for update verification.

## Example

For example, attach a signature created outside Bakery:

```toml
uses = "core/attach-bundle-signature"
inputs.bundle = { path = "release/update.rugixb" }
inputs.signature = { path = "release/signature.der" }
inputs.certificate = { path = "keys/bundle.crt" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
