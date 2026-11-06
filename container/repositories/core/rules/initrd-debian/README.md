## Behavior

Generate an initrd with a prepared Debian root filesystem's `mkinitramfs`.
The `kernel-version` input is a text file containing one installed kernel
version, optionally followed by a newline. Use `core/kernel-debian` to select
a kernel and produce this file.

The root filesystem must contain `mkinitramfs`, target kernel modules, and
the required initramfs hooks. Its programs must run on the builder, either
natively or through configured emulation. The rule uses a disposable copy and
leaves the input root filesystem unchanged.

The build source epoch is passed as `SOURCE_DATE_EPOCH`. Custom hooks must
also avoid clocks, randomness, and undeclared external state for reproducible
initrds.

## Example

For example, generate an initrd for a selected kernel:

```toml
uses = "core/initrd-debian"
inputs.rootfs = { path = "prepared-root" }
inputs.kernel-version = { path = "kernel-version" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. The output is `initrd`.
