# Inner Container Proof of Concept

This experiment tests Dockerfile-defined tooling environments inside Rugix
Bakery under rootless Podman. It builds an Alpine environment with OpenSSL, then
uses fresh working containers to sign an example artifact. It does not change
Bakery's recipes, configuration, or image assembly.

An environment image supplies a filesystem and image defaults, such as environment
variables and a working directory. Each invocation receives a writable container
filesystem derived from that image. Inputs, outputs, caches, and a temporary signing
key are passed as separate mounts. Removing a working container discards its
private changes while retaining explicitly mounted outputs and caches.

## Run the Experiment

Use Linux with rootless Podman and a local Bakery image that includes
`rugix-isolate`. By default, the runner uses
`ghcr.io/rugix/rugix-bakery:dev`, normally built with `mise run build`.
The image must match the machine's execution architecture. Network access is
needed for Debian packages, Alpine, and OpenSSL. FUSE mode also needs access to
`/dev/fuse`.

From the repository root:

```sh
bash xtask/inner-container/run.sh
```

The runner builds a separate PoC image on top of Bakery and tests three storage
modes. Each mode runs twice in separate outer containers sharing a dedicated
cache volume. It requires rootless Podman and runs without `--privileged`,
added capabilities, security-profile overrides, or a host container socket.
Only FUSE mode passes through `/dev/fuse`.

Run selected modes, for example:

```sh
bash xtask/inner-container/run.sh native vfs
```

Use a different Bakery image, for example:

```sh
BAKERY_IMAGE=localhost/my-bakery:dev bash xtask/inner-container/run.sh native
```

The runner prints a results directory under `build/inner-container-poc` and
the dedicated volume name. Logs include runtime versions, namespace mappings,
storage details, cache reuse, and assertion results. A failed requested mode makes
the runner exit with a nonzero status after trying the other modes.

The volume is retained for inspection; remove it with the command printed by
the runner. Set `POC_VOLUME` to reuse a particular volume. `POC_IMAGE` changes
the PoC image tag, and `POC_RESULTS_DIR` changes the log directory. Use a separate
volume for each concurrent runner.

## Storage and Namespace Setup

Buildah builds the inner environment images and manages their working containers
without a daemon. Podman supplies the surrounding Bakery container.

The PoC enters Bakery's existing user and mount namespace setup through
`rugix-isolate`. It preserves the outer container's UID/GID mappings, which
allows files with different Linux owners to be created. Buildah uses
`--isolation chroot --userns host` to reuse that user namespace. Here, `host`
means the namespace containing Buildah, inside Bakery.

Buildah stores image layers and writable container layers under
`/run/rugix/bakery/cache/inner-container-poc/<mode>/storage`. The runner mounts
a dedicated Podman volume at `/run/rugix/bakery/cache`, keeping this data outside
Bakery's overlay root filesystem. Buildah's temporary runtime state lives under
`/run/buildah-<mode>` and is recreated with each outer container.

| Mode | Storage | Device Access |
| --- | --- | --- |
| `native` | Kernel OverlayFS on the mounted volume | No FUSE device |
| `fuse` | Overlay storage through `fuse-overlayfs` | `/dev/fuse` |
| `vfs` | Directory copies instead of overlay mounts | No FUSE device |

The native test checks both Buildah's reported native-overlay support and the
actual working filesystem type. It fails if Buildah selects FUSE automatically.
Each mode uses its own store; the inner store is separate from Podman's outer
container store.

Chroot isolation controls command execution independently of the storage driver.
It shares several surrounding namespaces and is intended here for trusted build
tooling. The signing-key check establishes mount lifetime, not protection against
malicious tooling that deliberately copies or exposes the key. Buildah may leave
an empty file at a file mount's destination. The check requires that key contents
are inaccessible and the destination is no longer a mount; it permits that empty
placeholder.

## Checks

Each mode checks:

- Dockerfile `RUN` and `COPY`, including installing OpenSSL.
- Reuse of an identical environment within one outer container and after
  recreating that container.
- Cache invalidation when a copied context file changes. A random token in a
  dependent `RUN` instruction distinguishes re-execution from cache reuse.
- Image environment variables and working-directory defaults.
- Rejection of writes through a read-only input mount.
- Signing with a temporary, read-only key mount and verification outside the
  working container.
- Output ownership with UID 1234 and GID 2345.
- Fresh private filesystems and retention of explicitly mounted caches.
- Concurrent commands in separate working containers sharing an image store.
  A bounded rendezvous requires both commands to overlap.
- End of access to input, output, and key mounts before a subsequent command
  on the original signing container.
- Propagation of exit status 37 from a failing command.
- Removal of all working containers after a successful run.

The key is generated for each test and removed with temporary test data. It is
never copied into the Dockerfile context or committed into an environment image.

## Observed Results

On 2026-10-26, all three modes passed both outer-container invocations on
Linux 6.18.39, amd64, rootless Podman 5.8.4, and a Btrfs-backed local volume.
The PoC image used Debian Bookworm's Buildah 1.28.2 and fuse-overlayfs 1.10,
on top of Bakery image
`e485d7637f8eb675bd17d6a3346df19ed36682577f96d3f9716c32dcfac5983d`.

| Setup | Result |
| --- | --- |
| Native overlay on a dedicated volume, without `/dev/fuse` | Passed |
| FUSE overlay on a dedicated volume, with `/dev/fuse` | Passed |
| VFS on a dedicated volume, without `/dev/fuse` | Passed |
| Native mode with storage inside Bakery's overlay root filesystem | Native support reported false; test stopped before the working-filesystem check |
| Native mode on a volume, bypassing `rugix-isolate` | Image construction worked; explicit `buildah mount` failed with `EPERM` |

The last two rows are additional controls, separate from the runner's default
matrix. For example, after building the PoC image, probe storage inside the
outer root filesystem with:

```sh
podman run --rm \
    --env POC_STORE_BASE=/var/lib/inner-container-poc \
    localhost/rugix-bakery-inner-poc:dev native 1
```

The volume-backed native result supports using Bakery's namespace helper and a
dedicated environment store. The controls identify limitations of this setup
without establishing that every Buildah operation needs the helper.

## Limits

A successful run qualifies the tested kernel, runtime, backing filesystem, and
package versions. It does not establish compatibility with older kernels,
SELinux or AppArmor configurations, Docker, macOS, or Windows. The PoC does not
measure storage performance or test cancellation and crash recovery.

Environment images and packages use moving repositories and tags. Logs capture
the selected images and installed versions; this experiment does not establish
reproducible environment construction. It also does not implement EFI or
Raspberry Pi secure boot. The OpenSSL example exercises tooling and artifact
access without requiring either boot flow.
