Scan a root filesystem with Syft and emit SPDX and CycloneDX software inventories.
The build context supplies the system name, release version, and source date.

For example, a system composition can add this node:

```toml
[execution.rules.inventory]
uses = "core/sbom"
inputs.tree = { input = "rootfs" }
inputs.context = { input = "context" }

[execution.outputs]
"sbom.spdx.json" = { rule = { node = "inventory", output = "spdx" } }
"sbom.cdx.json" = { rule = { node = "inventory", output = "cyclonedx" } }
```

Document creation times use the source date. Document identifiers derive from
the inventory after timestamp normalization. Syft's version and package
catalogers come from the Bakery runtime and contribute to its cache identity.
The standard system pipeline publishes both inventory formats.
