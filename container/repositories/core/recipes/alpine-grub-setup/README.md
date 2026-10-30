## Behavior

Installs the LTS kernel and mkinitfs configuration, copies the kernel and initrd into the boot tree, and installs the second-stage GRUB configuration. Select a compatible generic GRUB target when assembling the system.

The custom mkinitfs feature supplies a mount implementation that resolves PARTUUID device references, as required by Rugix Ctrl.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/alpine-grub-setup"]
```
