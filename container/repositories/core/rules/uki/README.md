## Behavior

A Unified Kernel Image (UKI) combines an EFI stub, Linux kernel, initrd,
authenticated command line, and OS identification in one EFI executable.
This rule assembles those explicit inputs with Rugix Bakery's `ukify`.
The stub and kernel must match the target architecture.

The exported `image` is unsigned. Use `core/sign-efi` before deploying it on
a system that enforces UEFI Secure Boot. The command-line and OS identification
inputs are text files.

## Example

For example, assemble prepared boot artifacts:

```toml
uses = "core/uki"
inputs.stub = { path = "boot/stub.efi" }
inputs.kernel = { path = "boot/kernel" }
inputs.initrd = { path = "boot/initrd" }
inputs.cmdline = { path = "boot/cmdline" }
inputs.os-release = { path = "boot/os-release" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
