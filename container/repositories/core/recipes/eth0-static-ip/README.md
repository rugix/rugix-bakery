## Behavior

Writes /etc/network/interfaces.d/eth0. The target system must use ifupdown and name the intended interface eth0.

## Example

For example, add the recipe to a layer derived from a compatible base:

```toml
recipes = ["core/eth0-static-ip"]

[parameters."core/eth0-static-ip"]
address = "192.0.2.10"
gateway = "192.0.2.1"
netmask = "255.255.255.0"
```
