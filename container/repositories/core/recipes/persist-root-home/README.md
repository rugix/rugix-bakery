## Behavior

Installs a Rugix state configuration for /root. Files stored there then follow the system's persistent-state lifecycle.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/persist-root-home"]
```
