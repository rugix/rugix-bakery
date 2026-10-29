## Behavior

Adds Raspberry Pi's package repository, installs its kernel and boot tooling, and copies the resulting firmware into the boot tree. The recipe's boot configuration must match the image assembly used by the system.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-debian-setup"]
```
