## Behavior

Prepare the contents of a signed Raspberry Pi 4/CM4 or Pi 5 boot image. The `boot`
input supplies `start4.elf`, `fixup4.dat`, device trees (`*.dtb`), and an
`overlays` directory. The rule copies these with the supplied kernel, initrd,
and authenticated command line.

The `config` input supplies the complete inner `config.txt`. It is copied
verbatim, so board settings and selected overlays remain under the caller's
control. Select `kernel8.img` and load `initrd`; those are the filenames used
for the kernel and initrd inputs. For example:

```ini title="config.txt"
[all]
arm_64bit=1
kernel=kernel8.img
initramfs initrd followkernel
dtoverlay=my-board
```

UART is enabled only when the supplied configuration requests it.

The exported `tree` is unsigned. Package it with `core/image-fat` and sign
that image with `core/sign-rpi`. Firmware trust provisioning remains separate.

## Example

For example, prepare a Pi boot tree:

```toml
uses = "core/prepare-rpi-boot"
inputs.boot = { path = "firmware" }
inputs.kernel = { path = "kernel" }
inputs.initrd = { path = "initrd" }
inputs.cmdline = { path = "cmdline" }
inputs.config = { path = "config.txt" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
