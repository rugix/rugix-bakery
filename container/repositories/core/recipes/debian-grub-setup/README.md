## Behavior

Installs a target kernel, optional firmware, and the second-stage GRUB configuration. It copies the kernel and initrd into the boot tree. Select a compatible generic GRUB target when assembling the system.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/debian-grub-setup"]
```
