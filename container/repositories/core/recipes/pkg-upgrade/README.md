## Behavior

Includes pkg-update and runs after package lists are refreshed. Repository configuration determines the versions installed in the image.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/pkg-upgrade"]
```
