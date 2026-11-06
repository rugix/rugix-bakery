## Behavior

Select an unsigned systemd EFI stub from Rugix Bakery. The required
`architecture` parameter accepts `amd64` or `arm64` and must match the kernel.
Rugix Bakery includes resources for both architectures, independent of the
builder's architecture.

Supply the exported `stub` to `core/uki` with a kernel, initrd, command line,
and OS identification file.

## Example

For example, select an ARM64 stub:

```toml
uses = "core/efi-stub"
parameters.architecture = "arm64"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
