## Behavior

The manifest chooses payload paths, slot names, and encodings. The rule does not
impose a boot scheme or partition layout. It uses Rugix Bundler installed in
Bakery; the Bakery container identity contributes to the build's cache identity.

## Example

For example, package a manifest and its payload directory. The
[bundle format guide](https://rugix.org/docs/ctrl/next/updates/update-bundles/#building-a-bundle)
explains manifest fields and payload encodings:

```toml
uses = "core/bundle-rugix"
inputs.manifest = { file = "release/rugix-bundle.toml" }
inputs.payloads = { file = "release/payloads" }
```

Save the invocation as `build-rule.toml` in the project. Run
`./run-bakery bake rule build-rule.toml build/result`; exported artifacts appear
under their output names in `build/result`.
