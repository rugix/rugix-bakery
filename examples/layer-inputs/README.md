# Rule Outputs in Layers

This example compiles a static application in a tooling environment, installs it
through a recipe, and builds an ext4 image from the resulting layer. It targets
an amd64 builder. The image contains a minimal filesystem with the application;
it does not contain a bootable Linux system.

Use a Bakery image built from this checkout. Set `RUGIX_BAKERY_IMAGE` to that
image and `RUGIX_DEV=true` when using a local development image.

From this directory, build the image:

```sh
./run-bakery bake image demo
```

The result is `build/demo/system.img`. The installation recipe defines a local
`rules.hello` invocation and uses its output as the default for its `hello`
input. Bakery builds the application before the recipe's steps run, then exposes
it at `$RECIPE_INPUTS_DIR/hello`. The layer only needs to select the recipe.

To install a prebuilt binary instead, override that input in the layer:

```toml
[inputs.install-hello]
hello = { file = "prebuilt/hello" }
```

With this binding, Bakery skips the recipe's compiler rule. The binary path is
relative to the repository containing the layer.

To inspect the layer independently, export it through a layer rule:

```sh
./run-bakery bake rule layer.toml build/layer
./build/layer/rootfs/usr/bin/hello
```

Changing `application/main.c` reruns compilation. Changes to the resulting
binary's contents or metadata invalidate the layer and image.
Unrelated project files do not invalidate these nodes.

For an existing distribution, replace `root = true` with a parent such as
`parent = "core/debian-bookworm"`. Recipes and their input bindings work the
same way with an inherited filesystem.
