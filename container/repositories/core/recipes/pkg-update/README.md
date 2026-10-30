## Behavior

Runs early and includes pkg-cleanup as a dependency, so temporary package metadata is removed after the other recipes finish.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/pkg-update"]
```
