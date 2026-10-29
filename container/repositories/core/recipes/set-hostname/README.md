## Behavior

Writes the selected name into /etc/hostname in the target root filesystem.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/set-hostname"]

[parameters."core/set-hostname"]
hostname = "rugix-device"
```
