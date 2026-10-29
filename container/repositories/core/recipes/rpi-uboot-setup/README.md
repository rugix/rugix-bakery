## Behavior

Copies Bakery's bundled Raspberry Pi second-stage U-Boot script into the layer's boot tree. The surrounding image assembly must supply the compatible first-stage boot configuration.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-uboot-setup"]
```
