Patch an initialized Rugix boot tree against an existing disk's partition table.
The rule reads the disk and copies the boot tree before modifying it.

For example:

```toml
uses = "core/prepare-boot"
inputs.image = { path = "layout.img" }
inputs.boot = { path = "initialized-boot" }
parameters.target = "rpi-tryboot"
parameters.source-date-epoch = 1792972800
```

The predefined boot flows use root partition 5 for MBR and partition 4 for GPT.
The rule patches the Raspberry Pi kernel command line or GRUB environment with
that partition's identifier. It leaves partition identifiers and geometry
unchanged. The `bsp` and `unknown` targets retain the supplied boot content.

Run `core/initialize-boot` before resolving capacities so generated boot files
are included in size estimates. Both rules clamp modification times to the
source date and keep their input trees unchanged. The initialized configuration
tree passes directly to filesystem creation.
