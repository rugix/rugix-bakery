## Behavior

Installs a Rugix state configuration for /etc/fake-hwclock.data. It does not install fake-hwclock itself.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/persist-fake-hwclock"]
```
