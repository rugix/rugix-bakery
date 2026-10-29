## Behavior

Disables the Raspberry Pi OS user-configuration and one-time resize services, removes userconf-pi, and disables automatic initramfs handling in the boot configuration. It runs early so subsequent recipes can configure the final system.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-raspios-cleanup"]
```
