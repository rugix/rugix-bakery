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

The exported `layout` describes the factory partitions. The `partitions` output
contains the numbered factory artifacts for `core/image-gpt`. The target drive must have room for the
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

## Secondary Configuration Partition

Set `parameters.secondary-config = true` to reserve an empty
`config-secondary` as partition 2, with the same capacity as `config`.
The initial `system-a` becomes partition 3. Bootstrap preserves both config
partitions, expands `system-a`, and creates `system-b` as partition 4 and
`data` as partition 5. Enable the matching
`secondary_config` option in `core/debian-efi-verified-setup`, or supply an
equivalent custom system configuration and bootstrap plan.

This option changes the factory layout. Existing installations need a separate
partition-layout migration before receiving a root configured for it; installing
an update bundle does not add the secondary partition.

The secondary partition uses the Linux partition type so firmware does not
discover it as another ESP. Its reserved space can stage a complete replacement
ESP, including signed systemd-boot, UKIs, and loader configuration, before a
partition-table switch. This rule only reserves space; it does not implement
the update, activation, or recovery operation.

For example:

```toml
uses = "core/layout-efi-ab"
inputs.system = { path = "system.img" }
inputs.config = { path = "esp.fat" }
parameters.secondary-config = true
```
