## Behavior

Installs OpenSSH and a systemd or OpenRC service that preserves SSH host keys across boots. The optional authorized keys are appended to /root/.ssh/authorized_keys; the system's SSH configuration still determines whether root login is allowed.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/ssh"]
```
