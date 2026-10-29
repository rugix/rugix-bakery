Resolve a Bakery `ImageLayout` into an unpopulated disk. This rule supports
GPT, MBR with extended partitions, and raw firmware written before partitions.
It preserves the layout model used by system image configuration and BSPs.

The `roots` input contains prepared filesystem trees under their root names.
The `layer` input supplies paths referenced by raw firmware blobs. For example,
a partition with `root = "system"` reads its size estimate from
`roots/system`, while a blob with `file = "bsp/firmware.bin"` reads that file
from the layer input.

A partition with an explicit size keeps that capacity. Otherwise, Bakery
estimates capacity from its tree, with a minimum of 64 MiB and 20% overhead.
The output contains partition tables and raw firmware, with empty partition
regions. Disk and partition identifiers derive from the layout and resolved
geometry. Raw firmware reservations must not overlap, exceed their reserved
capacity, or overwrite partition table metadata.

For example, this Bakery `ImageLayout` describes one ext4 root partition:

```toml title="layouts/device.toml"
type = "gpt"

[[partitions]]
root = "system"
filesystem = { type = "ext4" }
```

The layout records the intended filesystem; this rule resolves geometry without
creating that filesystem. An invocation binds the layout and its prepared trees:

```toml
uses = "core/image-layout"
inputs.layout = { file = "layouts/device.toml" }
inputs.roots = { file = "prepared-roots" }
inputs.layer = { file = "prepared-layer" }
```

Use `core/assemble-disk` to populate the partitions with separately built
filesystem artifacts.
