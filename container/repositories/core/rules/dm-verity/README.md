## Behavior

Create detached dm-verity authentication data for a filesystem or other data
image. The image must have a positive length divisible by `data-block-size`.
The rule verifies the generated hash tree before exporting `hash-tree` and
`root-hash`.

The default salt and verity superblock UUID derive from the data image's SHA-256
digest. `size = 0` allocates the required hash-tree capacity, rounded up to
1 MiB; an explicit size must fit the authentication data. Fixed inputs,
parameters, source epoch, and tooling produce identical output bytes.

A hash tree authenticates data only when a trusted boot component authenticates
its root hash. For example, embed `root-hash` in a signed UKI's command line.
Keep the data image and hash tree together throughout assembly and updates.
Use `core/append-verity` to package both in one artifact with exported hash
offset and data-length metadata.

## Example

For example, protect an existing SquashFS image:

```toml
uses = "core/dm-verity"
inputs.image = { path = "system.squashfs" }
parameters.hash-algorithm = "sha256"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. Outputs appear as
`build/result/hash-tree` and `build/result/root-hash`.
