## Behavior

Write the `content` parameter to a file without adding a newline. Compositions
can use its `file` output to turn configurable text into a file input for another
rule.

## Example

For example, define a configuration file:

```toml
uses = "core/write-text"
parameters.content = """
[all]
enable_uart=1
"""
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`. The exported file is
`build/result/file`.
