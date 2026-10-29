## Behavior

Requires systemd and a Rugix Ctrl release that provides daemon mode. The recipe installs its service and access configuration. Each capability is disabled by default; enable the operations required by the clients that will use the daemon.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/rugix-ctrl-daemon"]
```
