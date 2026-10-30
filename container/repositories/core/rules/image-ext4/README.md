## Behavior

Automatic sizing reserves space for file data and inodes, populates the filesystem,
and shrinks it with `resize2fs`. Choose an explicit capacity when a deployment
needs a fixed slot size. A tree that does not fit fails the build.

The default UUID and directory hash seed come from a sorted archive of the input
tree with normalized timestamps. Set `uuid` when another component needs a
particular filesystem identity. The rule does not select a root partition, boot
scheme, or update slot.

`source-date-epoch` fixes filesystem creation, access, and change times and clamps
file modification times. It defaults to zero. Identical tree metadata, parameters,
and Bakery tooling produce identical image bytes, including with automatic sizing.

Additional `mkfs-options` use shell-style quoting to separate arguments. Bakery
does not perform shell expansion. Options that request random values or current
timestamps override the rule's reproducible defaults.

## Example

For example, package a prepared root filesystem:

```toml
uses = "core/image-ext4"
inputs.tree = { path = "prepared-root" }
parameters.label = "system"
```

Save this invocation as `root.toml` and run
`./run-bakery bake rule root.toml build/root`. The image appears at
`build/root/image`.
