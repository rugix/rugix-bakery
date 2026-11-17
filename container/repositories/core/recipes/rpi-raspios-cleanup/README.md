## Behavior

Disables the Raspberry Pi OS user-configuration and one-time resize services, removes userconf-pi, and disables automatic initramfs handling in the boot configuration. It runs before `core/pkg-update` and `core/pkg-upgrade` so package operations use the adapted system.

Sets `MODULES=most` in `/etc/initramfs-tools/conf.d/rugix` before removing packages. This prevents initramfs generation from trying to detect the target hardware inside the build chroot. The setting remains in the image so package operations in later layers can also regenerate initramfs.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-raspios-cleanup"]
```
