## Behavior

Check a Raspberry Pi boot signature independently of the signer that created
it. The rule verifies `signature` over `image` with `public-key` using the
pinned upstream `rpi-eeprom-digest` tool and enforces the 180 MB RAM disk
budget of `boot.img`.

Compositions run this rule after every replaceable Raspberry Pi signer, such
as the `rpi-signer` rule parameter of
[`core/image-rpi-verified`](../image-rpi-verified/README.md).

## Example

```toml
uses = "core/verify-rpi"
inputs.image = { path = "boot.img" }
inputs.signature = { path = "boot.sig" }
inputs.public-key = { path = "keys/boot.pub" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
