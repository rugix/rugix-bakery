## Behavior

SquashFS determines the image size from its compressed contents. The rule uses
`mksquashfs` without appending to an existing image.

The build source epoch fixes the filesystem creation time and clamps file timestamps.
It defaults to zero. The rule uses one compression worker and reproducible
SquashFS construction. Identical input contents and metadata, parameters, source epoch, and Bakery tooling
produce identical image bytes.

Additional `mkfs-options` use shell-style quoting to separate arguments. Bakery
does not perform shell expansion. Options that request random values or current
timestamps override the rule's reproducible defaults.

## Example

For example, package application files with XZ compression:

```toml
uses = "core/image-squashfs"
inputs.tree = { path = "application-tree" }
parameters.compression = "xz"
```

Save the invocation as `build-rule.toml` in the project. Run
`./run-bakery bake rule build-rule.toml build/result`; exported artifacts appear
under their output names in `build/result`.

For this rule, the source epoch must be between `0` and `4294967295` Unix seconds
(inclusive), as required by the filesystem tooling.
