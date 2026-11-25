Populate the filesystem images of an unpopulated disk layout. This rule builds
one numbered filesystem image per partition that declares a filesystem, and
leaves the rest untouched: raw firmware regions, uninitialized partitions, and
extended container partitions produce no output.

The `layout` input is a Rugix Bakery `ImageLayout`. Each partition carries its
filesystem (`ext4`, `fat32`, or `squashfs`), an optional fixed size, and an
optional `root` pointing into the `roots` input. The `image` input is the
unpopulated disk image produced by `core/image-layout`; its partition table
determines the output names and the capacity of each partition. The `roots`
input contains prepared filesystem trees, addressed by the first-level
component of each partition root — a root `"system/data"` reads the tree in
`roots/system/data`. Partitions without a root are built from an empty tree.

Outputs are written to the `partitions` directory as raw images named by
their partition numbers — one file per filesystem partition. Hand those same
images to `core/assemble-disk` to populate the disk.

Each filesystem image is built with the same code that backs `core/image-ext4`,
`core/image-fat`, and `core/image-squashfs`: ext4 images are sparse files with
stable, recomputed UUIDs, squashfs images are built with gzip compression and
131072-byte blocks, and `mkfs`/`mcopy` invocations run under the build's source
date. The archive trees used by the pipeline for bundled payloads are built
by the caller, not by this rule.

For example, a composition that populates the partitions of an unpopulated
disk and hands them to the disk assembler:

```toml
[execution.rules.populate]
uses = "core/image-populate-partitions"
inputs.layout = { path = "prepared-layout.json" }
inputs.image = { rule = { node = "layout", output = "image" } }
inputs.roots = { path = "prepared-roots" }
```

When the rules composing a higher-level image already produce the layout and
roots, the caller uses this rule in place of the per-filesystem rules; the
builtin pipeline does exactly that, keeping `core/assemble-disk` as the final
step.
