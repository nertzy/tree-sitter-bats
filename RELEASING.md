# Releasing

Releases use explicit versions, curated changelog entries, and immutable `v<version>` tags. Pushing a matching tag runs [`.github/workflows/publish.yml`](.github/workflows/publish.yml), which:

1. Checks that every manifest has the tag's version, that the tag is on `main`, and that `CHANGELOG.md` has notes for the version.
2. Builds the Wasm module, Node.js binaries and Python wheels for six platforms, and the Python source distribution, and packages the Rust crate. It installs the packed npm tarball and parses a test with it.
3. Waits for approval of the `npm`, `crates`, and `pypi` deployment environments. It then stages the npm package, publishes the crate, and publishes the Python package, each through trusted publishing with no stored token.
4. Creates the GitHub Release from the version's changelog section, with the Wasm module and its build provenance attestation attached.

The npm package is staged, not published. Approve it with 2FA on npmjs.com, through the link in the job summary, or with `npm stage approve <stage-id>`. The npm trusted publisher may only stage, so CI can never publish to npm directly.

Each publish job skips a version that its registry already has, so rerunning a release, or tagging a version published by hand, never publishes twice.

Go and Swift users install from the tag; they have no registry to publish to.

Pull requests that change packaging files, and manual runs of the workflow, build and smoke-test every package without publishing anything.

## Prepare a release

1. Move the notes under `## [Unreleased]` in `CHANGELOG.md` into a new `## [<version>] - YYYY-MM-DD` section, leaving `## [Unreleased]` empty. Update the tag in the README's "Install" section.
2. Set the version everywhere:

   ```sh
   tree-sitter version <version>
   npm install --package-lock-only --ignore-scripts
   cargo update --workspace
   ```

3. Check the result:

   ```sh
   script/check-release
   script/release-notes <version>
   ```

4. Merge the change to `main` after CI passes.
5. Create a signed, annotated tag at the merged commit and push it:

   ```sh
   git tag --sign --message v<version> v<version> <merged-commit>
   git push origin v<version>
   ```

6. In the workflow run, approve the `npm`, `crates`, and `pypi` deployments, then approve the staged npm package.

The tag ruleset prevents moving or deleting a pushed release tag. When a job fails for a transient reason, rerun it; when a release needs a code change, fix it and release a new version.

## Trusted publishers

Every registry trusts the same repository, workflow, and matching environment:

| Registry | Repository | Workflow | Environment | Notes |
| --- | --- | --- | --- | --- |
| npm | `nertzy/tree-sitter-bats` | `publish.yml` | `npm` | Staging only: leave "Allow npm publish" off |
| crates.io | `nertzy/tree-sitter-bats` | `publish.yml` | `crates` | |
| PyPI | `nertzy/tree-sitter-bats` | `publish.yml` | `pypi` | |

Each environment requires a maintainer's approval and accepts only `v*` tags.

npm expires a new trusted publisher unless a publish uses it within two days, so add the npm trusted publisher no more than two days before pushing the tag that first uses it.

## Initial release

npm and crates.io accept a trusted publisher only for a package that already exists, so version 0.1.0 goes to them by hand:

1. Run the workflow manually on the release commit and download its `npm-package` artifact.
2. Publish that tarball with `npm publish <tarball> --access public --provenance=false`, and publish the crate with `cargo publish` from the release commit. Use short-lived tokens injected at runtime, never stored in a file.
3. Revoke both tokens, then configure the crates.io trusted publisher. Add the npm trusted publisher when preparing the next release, within two days of its tag.

PyPI accepts a pending trusted publisher before the project exists, so it needs no token. The `v0.1.0` tag run publishes to PyPI, skips npm and crates.io, and creates the GitHub Release.
