## Behavior

Installs and enables the rugix-auto-commit systemd service. This commits during normal boot rather than waiting for an application-specific health check.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rugpi-auto-commit"]
```
