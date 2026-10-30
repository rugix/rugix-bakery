## Behavior

Use this recipe in a root layer. A snapshot selects a dated Debian archive and disables its validity-date check. A custom mirror is passed to mmdebstrap as a trusted source. The components parameter applies to that custom mirror.

## Example

For example, add the recipe to a root layer (set `root = true` instead of a parent):

```toml title="layers/base.toml"
root = true
recipes = ["core/debian-bootstrap"]

[parameters."core/debian-bootstrap"]
suite = "trixie"
```
