## Behavior

Assemble a sparse GPT disk from a Rugix `ImageLayout` and already created
partition images. The layout accepts JSON or TOML. Files in `partitions` are
named by their one-based partition numbers, such as `1` and `2`.

Each layout entry can specify a GPT `name`, `type` GUID, and byte `size`.
Without a size, capacity comes from that partition's artifact. A fixed-size
partition can omit its artifact to reserve an empty slot. Artifacts must fit
their reserved capacities. Partition starts align to 1 MiB.

`size = 0` sizes the disk from its partitions. An explicit disk size must be
a multiple of 512 bytes and fit the layout.

This rule accepts GPT layouts and partition images. Use `core/image-layout`
with `core/assemble-disk` for layouts that create geometry from root trees or
include raw firmware blobs. Those operations also support MBR layouts.

## Example

For example, assemble two prepared partition images with automatic capacity.
Save this as `layout.toml`:

```toml
type = "gpt"
partitions = [
    { name = "system-a" },
    { name = "system-b" },
]
```

Put the system images at `partitions/1` and `partitions/2`, then save this
invocation as `build-rule.toml`:

```toml
uses = "core/image-gpt"
inputs.layout = { path = "layout.toml" }
inputs.partitions = { path = "partitions" }
```

Run `./run-bakery bake rule build-rule.toml build/result`. The output is `image`;
partition capacities come from the supplied images.
