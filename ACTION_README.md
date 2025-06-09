# Doksnet GitHub Action

A GitHub Action for verifying that documentation and code stay synchronized using the [doksnet](https://crates.io/crates/doksnet) tool.

## Usage

### Basic Usage (Recommended for CI/CD)

```yaml
name: Documentation-Code Sync Check

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  doc-sync-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Verify Documentation-Code Sync
        uses: Pulko/doksnet@v1
        # Uses default: command 'test' and fails on error
```

### Advanced Usage

```yaml
name: Documentation Workflow

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  verify-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Test Documentation Sync
        id: doksnet-test
        uses: Pulko/doksnet@v1
        with:
          command: 'test'
          version: '0.2.0'
          working-directory: './docs'
          fail-on-error: false
      
      - name: Handle results
        if: steps.doksnet-test.outputs.exit-code != '0'
        run: |
          echo "Documentation sync issues found:"
          echo "${{ steps.doksnet-test.outputs.result }}"
          echo "Consider running 'doksnet test-interactive' locally to fix"
```

### Multi-OS Testing

```yaml
name: Cross-Platform Doc Sync

on: [push, pull_request]

jobs:
  test-docs:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Verify docs sync on ${{ matrix.os }}
        uses: Pulko/doksnet@v1
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `command` | Doksnet command to run | No | `test` |
| `version` | Version of doksnet to use | No | `latest` |
| `working-directory` | Working directory to run doksnet in | No | `.` |
| `fail-on-error` | Whether to fail the workflow if doksnet finds issues | No | `true` |

### Available Commands

- `test` - Verify all mappings (recommended for CI/CD)
- `new` - Initialize a .doks file (for setup)
- `remove-failed` - Remove all failed mappings (for cleanup)

**Note:** Interactive commands (`add`, `edit`, `test-interactive`) are not suitable for CI/CD environments.

## Outputs

| Output | Description |
|--------|-------------|
| `result` | Full output from the doksnet command |
| `exit-code` | Exit code from the doksnet command (0 = success, 1 = failure) |

## Examples

### Fail Build on Documentation Drift

```yaml
- name: Enforce Documentation Sync
  uses: Pulko/doksnet@v1
  with:
    command: 'test'
    fail-on-error: true  # Default - will fail CI if docs are out of sync
```

### Warning Only (Don't Fail Build)

```yaml
- name: Check Documentation Sync (Warning Only)
  uses: Pulko/doksnet@v1
  with:
    command: 'test'
    fail-on-error: false
```

### Specific Version

```yaml
- name: Use Specific Doksnet Version
  uses: Pulko/doksnet@v1
  with:
    version: '0.2.0'
```

### Custom Working Directory

```yaml
- name: Check Docs in Subdirectory
  uses: Pulko/doksnet@v1
  with:
    working-directory: './documentation'
```

## Workflow Integration Patterns

### 1. Gate for Pull Requests

```yaml
name: PR Checks

on:
  pull_request:
    branches: [ main ]

jobs:
  documentation-sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Pulko/doksnet@v1
        # Blocks PR merge if docs are out of sync
```

### 2. Nightly Documentation Health Check

```yaml
name: Nightly Doc Health

on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM daily

jobs:
  health-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Pulko/doksnet@v1
        with:
          fail-on-error: false
      # Could add Slack/email notifications here
```

### 3. Release Preparation

```yaml
name: Pre-Release Checks

on:
  push:
    tags:
      - 'v*'

jobs:
  verify-release-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Ensure docs are synchronized for release
        uses: Pulko/doksnet@v1
        # Critical check before releasing
```

## Setup Requirements

1. **Have a `.doks` file** in your repository (created with `doksnet new`)
2. **Add documentation-code mappings** (created with `doksnet add`)
3. **Use the action** in your workflows

## Performance Notes

- **Caching**: The action automatically caches Cargo dependencies for faster subsequent runs
- **Installation**: Installs doksnet from crates.io (fast, reliable)
- **Cross-platform**: Works on Linux, Windows, and macOS runners

## Troubleshooting

### Common Issues

**No .doks file found:**
```yaml
# Add a setup step if needed
- name: Initialize doksnet (if needed)
  run: |
    if [ ! -f .doks ]; then
      cargo install doksnet
      doksnet new
    fi
- uses: Pulko/doksnet@v1
```

**Custom Rust toolchain:**
```yaml
- uses: actions-rs/toolchain@v1
  with:
    toolchain: stable
    override: true
- uses: Pulko/doksnet@v1
```

## Related

- [Doksnet CLI Tool](https://crates.io/crates/doksnet)
- [Documentation](https://docs.rs/doksnet)
- [Source Code](https://github.com/Pulko/doksnet) 