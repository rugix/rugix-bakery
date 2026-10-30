## Behavior

Imports the configured Raspberry Pi OS Lite image for arm64. The source URL pins a dated image rather than following the newest release. Add core/rpi-raspios-setup when adapting the imported system for Rugix.

## Example

For example, choose this base for a project layer:

```toml
parent = "core/raspios-trixie"
```
