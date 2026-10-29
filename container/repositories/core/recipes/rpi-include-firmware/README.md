## Behavior

Copies the bundled EEPROM update and recovery files into the layer's configuration tree. For pi4 it also includes the USB controller firmware update. This packages the versions bundled with Bakery; it does not fetch a newer firmware release or enroll Secure Boot keys.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rpi-include-firmware"]

[parameters."core/rpi-include-firmware"]
model = "pi4"
```
