## Behavior

Runs late so other recipes can install packages first. On APT-based systems, set autoremove when the final filesystem should also drop packages no longer required by installed packages. Alpine ignores this option.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/pkg-cleanup"]
```
