# Archflow CLI Distribution and Release Operations

This document standardizes how to build, publish, install, and update the `archflow` CLI.
It is the baseline operating procedure for Phase10 Task1.

## 1. Release deliverables

Every release must publish all of the following assets on GitHub Releases:

- `archflow-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
- `archflow-vX.Y.Z-aarch64-unknown-linux-gnu.tar.gz`
- `archflow-vX.Y.Z-x86_64-apple-darwin.tar.gz`
- `archflow-vX.Y.Z-aarch64-apple-darwin.tar.gz`
- `*.sha256` for each archive
- `checksums.txt` (combined checksum list)

The release workflow is:

- `.github/workflows/archflow-release-cli.yml`

## 2. Release flow

1. Prepare a release notes file from template:
   - copy `.github/release-notes/template.md`
   - save as `.github/release-notes/vX.Y.Z.md`
2. Fill in required sections:
   - `## Migration Guide`
   - `## Supported Versions`
3. Create and push release tag:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

4. The release workflow builds cross-platform binaries, runs smoke checks, packages archives, generates checksums, and publishes GitHub Release assets.

## 3. Installation channels

Current official channel:

- GitHub Releases binary archives (recommended)

Planned channels (phased rollout):

- Homebrew
- Scoop
- apt/yum

Package manager channels must use the exact same artifact checksums from GitHub Releases.

## 4. Quick start install (recommended)

Use the official installer script (Linux/macOS):

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/archflow/main/scripts/install-archflow.sh | bash
```

Install a fixed version:

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/archflow/main/scripts/install-archflow.sh | bash -s -- vX.Y.Z
```

The script automatically:

- resolves target OS/arch
- downloads the correct release archive
- verifies SHA256 checksum
- installs `archflow` to `/usr/local/bin` (uses `sudo` only if needed)

Manual install steps remain available for debugging or constrained environments.

## 5. CI pinned-version setup

Use a fixed release version in CI to avoid accidental breaking changes.

Example (Linux runner):

```bash
ARCHFLOW_VERSION=vX.Y.Z

if [[ "$(uname -m)" == "aarch64" || "$(uname -m)" == "arm64" ]]; then
   ARCHFLOW_TARGET=aarch64-unknown-linux-gnu
else
   ARCHFLOW_TARGET=x86_64-unknown-linux-gnu
fi

curl -fsSL -o archflow.tar.gz "https://github.com/Arcflect/archflow/releases/download/${ARCHFLOW_VERSION}/archflow-${ARCHFLOW_VERSION}-${ARCHFLOW_TARGET}.tar.gz"
curl -fsSL -o archflow.tar.gz.sha256 "https://github.com/Arcflect/archflow/releases/download/${ARCHFLOW_VERSION}/archflow-${ARCHFLOW_VERSION}-${ARCHFLOW_TARGET}.tar.gz.sha256"
sha256sum -c archflow.tar.gz.sha256
tar -xzf archflow.tar.gz
chmod +x archflow
mv archflow "$RUNNER_TEMP/archflow"
"$RUNNER_TEMP/archflow" --version
"$RUNNER_TEMP/archflow" init --preset generic-layered --project-name ci-smoke
"$RUNNER_TEMP/archflow" plan
"$RUNNER_TEMP/archflow" audit
```

Cache strategy:

- cache by key `${ARCHFLOW_VERSION}-${ARCHFLOW_TARGET}`
- invalidate cache only when upgrading `ARCHFLOW_VERSION`

## 6. Update policy

Version support policy:

- supported for regular updates: latest + one prior minor line
- security patch priority: latest line first

When a release includes breaking changes, release notes must include:

- affected commands/options
- old -> new command examples
- rollback guidance

## 7. Verification checklist

After a release is published, verify at least once in a fresh environment:

- `archflow --version`
- `archflow init --preset generic-layered --project-name smoke`
- `archflow plan`
- `archflow audit`

If any step fails, mark release as draft or publish a follow-up patch release.
