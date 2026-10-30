# Cargo Packages as Layer Inputs

This example builds a Cargo application into a Debian package in a tooling
environment. A recipe installs the package into a Debian Bookworm layer, and a
filesystem rule turns that layer into an ext4 image. It targets an amd64 builder.
The result is a filesystem image without a bootloader or kernel.

Use a Bakery image built from this checkout. Set `RUGIX_BAKERY_IMAGE` to that
image and `RUGIX_DEV=true` when using a local development image. Run the commands
below from this directory.

## Build and Inspect the Package

The `cargo-deb` rule takes a Cargo project as its `source` input. Its compiler
environment contains pinned Rust and cargo-deb versions. The example application
has no external Cargo dependencies, so it builds with `--frozen`. The rule sets
`SOURCE_DATE_EPOCH=0` for repeatable package timestamps and keeps Cargo's writable
build directory outside the read-only source input.

For example, build the application package and inspect its control metadata:

```sh
./run-bakery bake rule package.toml build/package
./run-bakery envs shell compiler -- dpkg-deb --info /project/build/package/package.deb
```

The package includes the `rugix-hello` executable, documentation, license
information, and runtime dependency metadata. Its packaging settings live in
`application/Cargo.toml` under `package.metadata.deb`.

## Build and Inspect the Filesystem

The `install-hello` recipe declares a `package` input and a `build-package` node
that produces it:

```toml
[inputs]
package = "file"

[rules.build-package]
uses = "cargo-deb"
inputs.source = { path = "application" }

[input-defaults]
package = { rule = { node = "build-package", output = "package.deb" } }
```

The `path` binding supplies the existing `application` directory as the rule's
`source` input. The default binds the node's `package.deb` output to the recipe's
`package` input. Node names and input names are independent.

Bakery builds the package before the recipe's install step runs, then exposes it at
`$RECIPE_INPUTS_DIR/package`. The step runs `dpkg -i` inside the layer, so the
installed package is also recorded in Debian's package database.

Build the filesystem image, then export its layer for inspection:

```sh
./run-bakery bake image demo
./run-bakery bake rule layer.toml build/layer
./run-bakery envs shell compiler -- chroot /project/build/layer/rootfs dpkg-query -W rugix-hello
./run-bakery envs shell compiler -- chroot /project/build/layer/rootfs /usr/bin/rugix-hello
```

The filesystem image is `build/demo/system.img`. Changing the Cargo source,
lockfile, or package metadata rebuilds the package and its consumers. Unrelated
project files do not invalidate those nodes.

## Supply a Prebuilt Package

For example, override the recipe input in `layers/demo.toml`:

```toml
[inputs.install-hello]
package = { path = "prebuilt/rugix-hello.deb" }
```

Bakery then skips the recipe's package-building rule. The package path is
relative to the repository containing the layer. The prebuilt package must target
amd64 and have its runtime dependencies available in the layer.
