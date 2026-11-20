# Releasing Rugix Bakery

Pushing a release tag runs the Build workflow. It checks formatting, dependency
policy, unit tests, test tooling, and system tests, including the extended tests.
Clippy also runs, but currently remains advisory because of a compiler crash.
CI builds Linux binaries and SBOMs for AArch64 and x86-64 and a container image
for both architectures. It publishes the container tags only after checks and
system tests succeed, then creates a draft GitHub release, uploads its assets
and `SHA256SUMS`, and publishes it.

Release tags use `vMAJOR.MINOR.PATCH`. Prereleases add `-alpha.N`, `-beta.N`,
`-dev.N`, or `-rc.N`, for example `v0.10.0-rc.1`. CI marks these as GitHub
prereleases and leaves the latest stable GitHub release and stable container
channels unchanged.

## Prepare and Preview

Choose a version based on the changes being released. In `CHANGELOG.md`, move
the changes from `Unreleased` into a matching heading, for example
`## Version 0.10.0`, and leave an empty `Unreleased` section for future changes.
For a prerelease, use the full version, such as `## Version 0.10.0-rc.1`.

For a new stable minor version, update the default `RUGIX_VERSION` in
`container/run-bakery` to the corresponding channel, for example `v0.10`.
Update the Bakery documentation in
[rugix-docs](https://github.com/rugix/rugix-docs) for the release, including
installation instructions and schemas. Launcher download URLs use a Git
branch such as `v0.10`. The release tasks push only the exact release tag;
promote the branch after verifying the published stable release as described below.

Commit the preparation and merge it into `main`. With Python 3.11 or newer,
mise, and an authenticated GitHub CLI available, preview the checked-out release
commit. For example:

```sh
mise run release:preview v0.10.0
```

The preview fetches tags, validates the version and committed changelog section,
and shows the commit, previous release, asset list, container reference, and notes.
It does not create a tag or release. A version must be newer than every release
tag in its ancestry. Stable notes compare against the highest earlier stable
version; prerelease notes also consider earlier prereleases.

Release notes contain the matching changelog section followed by GitHub's list
of merged pull requests. If generating that list fails, the preview still shows
the local plan and explains the failure. Publication requires both parts.

To exercise CI before tagging, manually run Build on the release commit's
branch, with `publish` left false. This runs checks and builds and publishes
branch container tags, but does not create a GitHub release. Tag builds also
run the extended system tests and embed the exact release tag in the binaries.

## Publish

Check out the latest `origin/main` commit with a clean working tree. For example,
to release version 0.10.0:

```sh
mise run release:tag v0.10.0
```

The task fetches tags and `main`, validates the release, creates an annotated tag,
and pushes only that tag. It refuses an existing tag, uncommitted changes, or a
checkout different from `origin/main`. No Cargo package version bump is needed.

Watch Build until Publish Release succeeds. Check the release notes, container
image, and downloaded assets before announcing the release. Assets are:

- `aarch64-unknown-linux-musl.tar` and `x86_64-unknown-linux-musl.tar`, each
  containing Bakery, the isolator, the bundled Rugix tools, and their SBOMs.
- `run-bakery`, the launcher from the tagged sources.
- `SHA256SUMS`, covering both archives and the launcher.

The versioned container is available at
`ghcr.io/rugix/rugix-bakery:<tag>`. To select an exact version with the launcher,
including a prerelease, set `RUGIX_VERSION`. For example:

```sh
RUGIX_VERSION=v0.10.0-rc.1 ./container/run-bakery --version
```

After verifying a stable release, create or advance its minor-version Git
branch to the released commit so launcher download URLs serve the matching
script. For example:

```sh
git push origin 'v0.10.0^{commit}:refs/heads/v0.10'
```

If the push is rejected because it would move the branch backward or replace
unrelated history, inspect the branch before proceeding.

## Recover a Failed Release

If a check or build fails, rerun the failed jobs after resolving the cause.
If the tagged sources need changes, prepare a new version and tag.

An upload failure leaves a draft release. Rerun the failed publishing job, or
manually run Build on the same release tag with `publish` set to true. For example:

```sh
gh workflow run build.yml --ref v0.10.0 -f publish=true
```

CI resumes the draft, replaces its asset files, and publishes only after all
uploads succeed. Publishing runs for the same tag are serialized. A published
GitHub release is left unchanged on subsequent runs. The build can still
republish container tags when rerun.

If the local tag was created but its push failed, inspect the remote before
retrying `git push origin refs/tags/TAG` with the chosen tag in place of `TAG`.
Keep the tag on the commit that was previewed.
