## Behavior

Check an EFI signature independently of the signer that created it. The rule
detaches the signature from `signed`, attaches it to the original `image`, and
verifies the result against `certificate` with `sbverify`. The exported `image`
is therefore the original executable with the returned signature. Verification
fails if the signature covers a different Authenticode digest or was created
with a key that `certificate` does not verify.

Compositions run this rule after every replaceable EFI signer, such as the
`efi-signer` rule parameter of
[`core/image-efi-verified`](../image-efi-verified/README.md).

## Example

For example, check an image signed outside Bakery:

```toml
uses = "core/verify-efi"
inputs.image = { path = "boot.efi" }
inputs.signed = { path = "boot.signed.efi" }
inputs.certificate = { path = "keys/boot.crt" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
