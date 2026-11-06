## Behavior

Create the factory GPT `ImageLayout` for the core Raspberry Pi verified boot
flow. The `system` input contains SquashFS with appended verity
data. Capacities come from the corresponding input images.

| Number | Name | Factory Contents |
| --- | --- | --- |
| 1 | `config` | Mutable configuration image |
| 2 | `boot-a` | Signed boot partition image |
| 3 | `boot-b` | Empty reservation matching `boot-a` |
| 4 | `system-a` | Combined root and verity image |

The first three partitions use the EFI System Partition type for the
firmware-visible FAT filesystems. The selection policy in
`core/prepare-rpi-config` uses boot partitions 2 and 3.

On first boot, Rugix Ctrl uses the matching recipe's bootstrap plan to preserve
the configuration and boot capacities, expand the system partition capacity,
create an empty `system-b` as partition 5, and create `data` as partition 6 in
the remaining drive space. The Debian recipe defaults each system slot to at
least 4 GiB.

Supply partition artifacts separately to `core/image-gpt`. The target drive
must have room for the completed layout, which is larger than the factory image.

## Example

For example, create a layout from prepared partition images:

```toml
uses = "core/layout-rpi-ab"
inputs.system = { path = "system.img" }
inputs.boot = { path = "boot.fat" }
inputs.config = { path = "config.fat" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
