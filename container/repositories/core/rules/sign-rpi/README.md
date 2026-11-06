## Behavior

Sign a FAT `boot.img` for Raspberry Pi 4/CM4 or Pi 5 using the pinned
`rpi-eeprom-digest` tool installed in Rugix Bakery. The rule requires an
RSA-2048 key and verifies that the supplied PEM public key matches the private
`key` secret. It verifies the generated `signature` against the image before
exporting it.

The input is the inner boot RAM disk, not its containing FAT partition. Images
larger than 180,000,000 bytes are rejected before signing. This conservative
limit stays within the firmware's documented
[180 MB RAM disk budget](https://www.raspberrypi.com/documentation/computers/config_txt.html#boot_ramdisk).
The outer boot partition can reserve more capacity without enlarging the RAM
disk loaded by firmware.

`signing-time` sets the signature timestamp as Unix seconds encoded as a
string. An empty value uses the build source epoch. Fixed image, key, timestamp,
and tooling produce identical signatures.

Place the result beside the authenticated image as `boot.sig` and `boot.img`.
The board must be provisioned to trust the matching key and enforce signed boot.
This rule does not provision EEPROM keys or program OTP fuses.

## Example

For example, sign a prepared boot image:

```toml
uses = "core/sign-rpi"
inputs.image = { path = "boot.img" }
inputs.public-key = { path = "keys/rpi.pub" }
secrets.key = { path = "keys/rpi.key" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.

The signing tool is distributed under its
[upstream license](https://github.com/raspberrypi/rpi-eeprom/blob/master/LICENSE).
