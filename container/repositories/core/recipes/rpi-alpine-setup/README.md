## Behavior

Installs the Raspberry Pi kernel, copies boot files into the layer's boot tree, and applies the supplied Raspberry Pi configuration. Use it with a compatible Raspberry Pi image target.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-alpine-setup"]
```
