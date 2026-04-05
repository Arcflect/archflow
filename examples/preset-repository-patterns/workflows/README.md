# GitHub Workflow Examples for Preset-Based Projects

These workflows are minimal examples for repositories that started from an
Archflow preset.

They are intentionally small and illustrative.

## Included examples

- `verify-preset-project.yml`
- `plan-scaffold-prompt-preview.yml`

## Assumed repository shape

The examples assume Archflow config files are under `archflow/`:

- `archflow/project.arch.yaml`
- `archflow/placement.rules.yaml`
- `archflow/contracts.template.yaml`
- `archflow/artifacts.plan.yaml`

If your repository keeps config files in a different location, adjust
`working-directory` accordingly.

## Installation step note

The workflows include a pinned Archflow binary installation step:

```bash
ARCHFLOW_VERSION="v0.1.0"
ARCHFLOW_TARGET="x86_64-unknown-linux-gnu"
curl -fsSL -o archflow.tar.gz "https://github.com/Arcflect/archflow/releases/download/${ARCHFLOW_VERSION}/archflow-${ARCHFLOW_VERSION}-${ARCHFLOW_TARGET}.tar.gz"
curl -fsSL -o archflow.tar.gz.sha256 "https://github.com/Arcflect/archflow/releases/download/${ARCHFLOW_VERSION}/archflow-${ARCHFLOW_VERSION}-${ARCHFLOW_TARGET}.tar.gz.sha256"
sha256sum -c archflow.tar.gz.sha256
tar -xzf archflow.tar.gz
chmod +x archflow
sudo mv archflow /usr/local/bin/archflow
archflow --version
```

Use a fixed version in CI and rotate intentionally. See also:

- `docs/release-operations.md`
