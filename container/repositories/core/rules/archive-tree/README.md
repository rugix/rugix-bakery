Create a sorted PAX tar archive of a filesystem tree. The archive preserves
ownership, permissions, hard links, symlinks, device nodes, ACLs, and extended
attributes. Access and change times are omitted, and modification times are
clamped to the source date.

For example:

```toml
uses = "core/archive-tree"
inputs.tree = { path = "prepared-root" }
parameters.source-date-epoch = 1792972800
```

This artifact can accompany a filesystem image for inspection or further
processing. Filesystem image rules consume directory trees directly; they do not
require an intermediate archive.
