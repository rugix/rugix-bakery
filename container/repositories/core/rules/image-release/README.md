## Behavior

Create a factory GPT disk and a signed Rugix update bundle in one composition.
`layout` and numbered `partitions` feed `core/image-gpt`; `manifest` and
`payloads` feed `core/bundle-rugix`. `core/sign-bundle-rugix` authenticates
the bundle using `certificate` and the private `key` secret.

Supply one PEM-encoded RSA signing certificate. The bundle embeds only this
signer certificate, without intermediates. Issue it directly from the root that
Rugix Ctrl trusts. The signer must not be a CA certificate.
Concatenated PEM certificate chains are not supported by this signing rule.

This composition joins those operations without imposing partition roles,
payload names, or a boot scheme. The caller connects the same artifacts to
factory partitions and bundle payloads. The manifest maps payload filenames to
the consuming system's Rugix Ctrl slots.

`size` is the disk capacity in bytes; zero sizes it from the layout.
`signing-time` is Unix seconds encoded as a string; an empty value uses the
build source epoch. Outputs are `image` and `bundle`.

## Example

For example, assemble an existing layout and payload set:

```toml
uses = "core/image-release"
inputs.layout = { path = "release/layout.toml" }
inputs.partitions = { path = "release/partitions" }
inputs.manifest = { path = "release/rugix-bundle.toml" }
inputs.payloads = { path = "release/payloads" }
inputs.certificate = { path = "keys/bundle.crt" }
secrets.key = { path = "keys/bundle.key" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.


The `bundle-hash` output contains the bundle identity used to verify updates.
