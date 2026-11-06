## Behavior

Create the factory GPT `ImageLayout` for the core EFI verified boot flow.
The `system` input contains SquashFS with appended verity data; `config` is an
EFI System Partition (ESP) image, which Rugix Ctrl uses as its configuration
partition. Their lengths determine factory capacities.

| Number | Name | Factory Contents |
| --- | --- | --- |
| 1 | `config` | ESP image |
| 2 | `system-a` | Combined root and verity image |

The factory disk ends at the initial A system partition. On first boot, Rugix
Ctrl uses the matching recipe's bootstrap plan to preserve the ESP, expand the
system partition capacity, create an empty `system-b` as partition 3, and
create `data` as partition 4 in the remaining drive space. The Debian recipe
defaults each system slot to at least 4 GiB.

The exported `layout` describes the factory partitions. Supply their artifacts
separately to `core/image-gpt`. The target drive must have room for the
completed layout, which is larger than the factory image.

## Example

For example, create a layout from prepared partition images:

```toml
uses = "core/layout-efi-ab"
inputs.system = { path = "system.img" }
inputs.config = { path = "esp.fat" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
