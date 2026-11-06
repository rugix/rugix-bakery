## Behavior

Select one kernel from a prepared Debian root filesystem. Kernel versions come
from directories in `lib/modules`; the corresponding image must exist at
`boot/vmlinuz-<version>`. Export the selected image as `kernel` and its
version, followed by a newline, as `version`.

An empty `version` accepts any installed version. An empty `flavor` accepts any
suffix. After applying both filters, exactly one kernel must remain. Set
`version` when a root filesystem contains several installed kernels.

## Example

For example, select a Raspberry Pi kernel:

```toml
uses = "core/kernel-debian"
inputs.rootfs = { path = "prepared-root" }
parameters.flavor = "rpi-v8"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. Supply its `version`
output to `core/initrd-debian`.
