## Behavior

Builds a fresh root filesystem with core/debian-bootstrap and selects the Bookworm suite. Extend this layer with application recipes and the boot setup required by the target device.

## Example

For example, choose this base for a project layer:

```toml
parent = "core/debian-bookworm"
```
