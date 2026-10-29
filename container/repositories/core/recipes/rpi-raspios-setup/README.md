## Behavior

Combines Rugix Ctrl installation, Raspberry Pi OS cleanup, and fake-hwclock persistence through recipe dependencies. Use it when extending an imported Raspberry Pi OS layer.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-raspios-setup"]
```
