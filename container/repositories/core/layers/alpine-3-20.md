## Behavior

Builds a fresh root filesystem with core/alpine-bootstrap and selects Alpine 3.20. Extend this layer with application recipes and the boot setup required by the target device.

## Example

For example, choose this base for a project layer:

```toml
parent = "core/alpine-3-20"
```
