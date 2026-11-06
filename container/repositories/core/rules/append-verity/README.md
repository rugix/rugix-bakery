## Behavior

Package a data image and its detached dm-verity hash tree in one artifact.
The combined image contains the original data, zero-filled alignment padding,
and the hash-tree image, including its verity superblock.

Supply a hash tree generated for the exact data image, for example with
`core/dm-verity`. Keep that rule's `root-hash` output for the authenticated
boot metadata. `alignment` defaults to 4096 bytes and must be a power of two
of at least 512 bytes that suits the hash block size.

| Output | Contents |
| --- | --- |
| `image` | Combined data and verity image |
| `hash-offset` | Byte offset of the appended hash area, as decimal text |
| `data-size` | Original data length in bytes, excluding padding and hashes, as decimal text |

The data image and hash tree must both be nonempty. A consuming boot flow must
authenticate the root hash and the geometry used to open the combined image.
`core/verified-cmdline` embeds the offset and data length in boot arguments
for the core verified-root contract.

## Example

For example, append authentication data to a prepared filesystem image:

```toml
uses = "core/append-verity"
inputs.image = { path = "system.squashfs" }
inputs.hash-tree = { path = "verity.img" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
