## Behavior

Extract the signed metadata of a Rugix update bundle. The metadata contains the
bundle header hash, which covers all payload hashes. A CMS signature over this
metadata with encapsulated content authenticates the bundle. This rule matches
`rugix-bundler signatures prepare`.

[`core/sign-bundle-rugix`](../sign-bundle-rugix/README.md) combines this rule
with a signer and [`core/attach-bundle-signature`](../attach-bundle-signature/README.md).

## Example

```toml
uses = "core/prepare-bundle-signature"
inputs.bundle = { path = "release/update.rugixb" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
