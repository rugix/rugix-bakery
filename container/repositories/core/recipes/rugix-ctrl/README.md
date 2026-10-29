## Behavior

The container source uses the binaries bundled with Bakery. A release series resolves the newest matching published release, including prereleases; an exact tag downloads that release. Pin an exact tag when a build must keep the same Rugix Ctrl version.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rugix-ctrl"]

[parameters."core/rugix-ctrl"]
version = "container"
```
