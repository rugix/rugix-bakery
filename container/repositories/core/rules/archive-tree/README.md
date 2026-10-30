Create a sorted PAX tar archive of a filesystem tree. The archive preserves
ownership, permissions, hard links, symlinks, device nodes, ACLs, and extended
attributes. Access and change times are omitted, and modification times are
clamped to the source date.

For example:

```toml
uses = "core/archive-tree"
inputs.tree = { path = "prepared-root" }
```

This artifact can accompany a filesystem image for inspection or further
processing. Filesystem image rules consume directory trees directly; they do not
require an intermediate archive.

The archive uses the build's `SOURCE_DATE_EPOCH`. To use a different modification
time limit, set `parameters.clamp-mtime` to an RFC 3339 timestamp, for example
`"2024-01-01T00:00:00Z"`.
