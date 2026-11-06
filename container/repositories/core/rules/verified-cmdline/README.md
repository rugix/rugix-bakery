## Behavior

Build the command line consumed by the core verified-root initramfs hooks.
Supply the root hash and appended-image geometry as text files:

| Input | Required Value |
| --- | --- |
| `root-hash` | A 64-digit SHA-256 hexadecimal hash |
| `hash-offset` | Hash-area byte offset, aligned to 4096 bytes and at least `data-size` |
| `data-size` | Positive authenticated data length in bytes, divisible by 4096 |

`core/image-squashfs-verity` exports these under the same names. The command line
includes `rugix.verity-root-hash`, `rugix.verity-hash-offset`, and
`rugix.verity-data-blocks`. The block count is
the data length divided by 4096. Embed the command line in a signed boot
artifact to authenticate both the root hash and the geometry used at boot.

`platform = "efi"` selects the boot group from systemd-boot's selected entry;
`platform = "rpi"` selects it from the firmware's boot partition. The command
line requests a read-only SquashFS root at `/dev/mapper/root`, selects
`init=/usr/bin/rugix-ctrl`, and disables systemd's automatic GPT discovery.
After the initramfs opens the verified root, Rugix Ctrl performs native
bootstrap and its root overlay and persistent-state setup.

`extra` adds authenticated arguments such as console settings. It rejects
`rw`, `ro`, `root=…`, `rootfstype=…`, `init=…`, `rdinit=…`, `rugix.…`,
and `panic=…`, as well as `break`, `break=…`, `rd.break`, and `rd.break=…`.
Newlines, carriage returns, and NUL bytes are also rejected.

## Example

For example, after exporting `core/image-squashfs-verity` to `build/root`,
prepare Raspberry Pi boot arguments from its metadata:

```toml
uses = "core/verified-cmdline"
inputs.root-hash = { path = "build/root/root-hash" }
inputs.hash-offset = { path = "build/root/hash-offset" }
inputs.data-size = { path = "build/root/data-size" }
parameters.platform = "rpi"
parameters.extra = "console=serial0,115200 console=tty1"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. The output is `cmdline`.

The `extra` parameter supplies caller-selected consoles and other kernel
arguments and defaults to empty. The `panic-timeout` parameter controls the
reboot delay after a kernel panic, in seconds. Zero disables that reboot;
the default is 10 seconds. Arguments that replace verified-root metadata,
select another init, or request an initramfs shell are rejected.
