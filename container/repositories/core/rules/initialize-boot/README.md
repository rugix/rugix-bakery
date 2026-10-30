Initialize one of Rugix's predefined boot flows before sizing or creating its
filesystems. The rule copies both input trees, installs bootloader or firmware
files into the configuration tree, and adds the bootstrap marker.

For example:

```toml
uses = "core/initialize-boot"
inputs.boot = { path = "prepared-boot" }
inputs.config = { path = "prepared-config" }
parameters.target = "generic-grub-efi"
parameters.architecture = "amd64"
parameters.source-date-epoch = 1792972800
```

The supported boot flows are GRUB EFI, Raspberry Pi tryboot, and Raspberry Pi
U-Boot. The `bsp` and `unknown` targets retain the supplied trees without
installing a predefined boot flow. Bootloader binaries and firmware come from
the Bakery runtime, which contributes to the rule's cache identity.

Use these output trees when resolving dynamic partition capacities. Once the
partition table exists, `core/prepare-boot` patches the root partition reference.
