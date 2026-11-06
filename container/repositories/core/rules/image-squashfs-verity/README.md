## Behavior

Create a SquashFS root image with appended dm-verity authentication data.
The composition runs `core/image-squashfs`, generates a detached hash tree with
`core/dm-verity`, then combines both with `core/append-verity`. A single
partition or update payload can therefore carry the filesystem and its
authentication data.

`compression` defaults to `none`. Uncompressed images store file contents as
is, which helps delta updates find unchanged data across releases. Select a
compressor such as `zstd` or `xz` for smaller images.

| Output | Contents |
| --- | --- |
| `image` | SquashFS followed by alignment padding and verity authentication data |
| `root-hash` | SHA-256 root hash as hexadecimal text |
| `hash-offset` | Byte offset of the appended hash area, as decimal text |
| `data-size` | SquashFS length in bytes before padding and hashes, as decimal text |

A trusted boot component must authenticate the root hash and geometry.
Supply `root-hash`, `hash-offset`, and `data-size` to `core/verified-cmdline`
when using the core verified-root boot contract.

## Example

For example, build a compressed root image from a prepared tree:

```toml
uses = "core/image-squashfs-verity"
inputs.tree = { path = "prepared-root" }
parameters.compression = "zstd"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
