## Behavior

Initialize mutable Raspberry Pi tryboot selection separately from signed
boot images. The `config` output contains `autoboot.txt`: partition 2 boots
by default and partition 3 is the trial slot. `tryboot_a_b=1` enables A/B
firmware behavior used by Rugix Ctrl's built-in `rpi-tryboot` flow.

The directory also contains `.rugix/bootstrap`. This marker asks Rugix Ctrl
to complete the target's partition layout during first boot, using the root
filesystem's bootstrap configuration. Package the exported directory as FAT.
The `config` input supplies layer files to preserve. The rule writes its own
`autoboot.txt` and bootstrap marker.

This policy matches `core/layout-rpi-ab` and
`core/debian-rpi-verified-setup`.

## Example

For example, generate the initial configuration tree:

```toml
uses = "core/prepare-rpi-config"
inputs.config = { tree = {} }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. The output is the
`config` directory.
