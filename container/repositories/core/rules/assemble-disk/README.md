Place independently built filesystem images into an existing disk layout. This
rule supports GPT and MBR, including logical partitions. It preserves partition
identifiers, offsets, empty partitions, and any raw firmware in the input disk.

For example, this composition node populates root partition 5 of an MBR disk:

```toml
[execution.rules.disk]
uses = "core/assemble-disk"
inputs.image = { rule = { node = "layout", output = "image" } }
inputs.partitions = { tree = { "5" = { rule = { node = "root", output = "image" } } } }
```

Partition images are named by their actual partition numbers. Every supplied
image must fit its partition. Omitted partitions retain their input contents;
extended partition containers cannot receive payloads. The rule writes a fresh
disk image without modifying its inputs or using loop devices.
