## Behavior

Use this recipe in a root layer. It downloads and extracts an Alpine minirootfs for the target architecture, then configures DNS. Supported architectures are amd64, arm64, armv7, and armhf.

## Example

For example, add the recipe to a root layer (set `root = true` instead of a parent):

```toml title="layers/base.toml"
root = true
recipes = ["core/alpine-bootstrap"]

[parameters."core/alpine-bootstrap"]
version = "3.20"
```
