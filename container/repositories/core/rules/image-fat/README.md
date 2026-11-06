## Behavior

Automatic sizing accounts for file clusters, directories, long filenames, and FAT
metadata and creates FAT32 images. Small trees still require FAT32's minimum
cluster count. For fixed capacities, the formatter selects FAT12, FAT16, or FAT32
to suit the image size. Use `mkfs-options = "-F 32"` to require FAT32.

With automatic sizing, `free-percent` reserves additional file-cluster capacity
as a percentage of the current contents. `minimum-size` sets a byte floor, and
`size-alignment` rounds the result up to a power-of-two byte multiple. All three
default to zero; zero alignment retains the rule's 1 MiB rounding. An explicit
`size` overrides automatic sizing and these adjustments.

Contained symlinks are dereferenced. Links outside the input tree, directory
cycles, dangling links, and special files are rejected.

The rule fails if the tree does not fit an explicit capacity. The default
volume ID comes from tree paths and file contents; set `volume-id` when another
component needs a particular ID.

The rule formats with invariant metadata and copies entries in sorted order.
The build source epoch fixes copied-entry timestamps, using UTC. It defaults to zero;
FAT timestamps cannot precede 1980. Identical inputs, parameters, source epoch, and Rugix Bakery
tooling produce identical image bytes.

Additional `mkfs-options` use shell-style quoting to separate arguments. Rugix Bakery
does not perform shell expansion. Options that request random values or current
timestamps override the rule's reproducible defaults.

## Example

For example, package an existing boot tree:

```toml
uses = "core/image-fat"
inputs.tree = { path = "boot-tree" }
parameters.label = "BOOT"
parameters.volume-id = "1234abcd"
```

Save the invocation as `build-rule.toml` in the project. Run
`./run-bakery bake rule build-rule.toml build/result`; exported artifacts appear
under their output names in `build/result`.

For this rule, the source epoch must be between `0` and `4294967295` Unix seconds
(inclusive), as required by the filesystem tooling.
