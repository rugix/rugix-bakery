## Behavior

Select an unsigned systemd-boot executable from Rugix Bakery. The required
`architecture` parameter accepts `amd64` or `arm64` and must match the target.
Rugix Bakery includes resources for both architectures, independent of the
builder's architecture.

Sign the exported `image` with `core/sign-efi` before installing it on an EFI
System Partition for UEFI Secure Boot.

## Example

For example, select an AMD64 bootloader:

```toml
uses = "core/systemd-boot"
parameters.architecture = "amd64"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
