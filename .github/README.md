# GitHub Actions Workflows

This directory contains GitHub Actions workflows for `doksnet` CI/CD automation.

## Workflows Overview

### 1. Quick Check (`quick-check.yml`)
**Triggers:** Every push to any branch
**Purpose:** Fast feedback for development

**What it does:**
- ✅ Check code formatting (`cargo fmt --check`)
- ✅ Run cargo check for compilation errors
- ✅ Run clippy for code quality (warnings only)
- ✅ Run unit tests (`cargo test --bin doksnet`)

**Duration:** ~1-2 minutes
**Use case:** Quick validation during development

### 2. CI (`ci.yml`)
**Triggers:** Push to `main`/`develop` branches, Pull Requests
**Purpose:** Comprehensive testing and quality assurance

**What it does:**
- ✅ Code formatting check
- ✅ Clippy with strict warnings (`-D warnings`)
- ✅ Cargo check for all targets
- ✅ Complete test suite (unit, integration, command, end-to-end tests)
- ✅ Multi-platform builds (Linux, Windows, macOS)
- ✅ Security audit with `cargo-audit`
- ✅ Code coverage generation and upload to Codecov

**Duration:** ~5-10 minutes
**Use case:** Gate for merging PRs and releases

### 3. Release (`release.yml`)
**Triggers:** Git tags starting with `v*` (e.g., `v1.0.0`)
**Purpose:** Automated releases and publishing

**What it does:**
- 🚀 Create GitHub release
- 📦 Build binaries for multiple platforms:
  - Linux (x86_64)
  - Windows (x86_64)
  - macOS (x86_64 and ARM64)
- 📤 Upload release artifacts
- 📮 Publish to crates.io (optional)

**Duration:** ~10-15 minutes
**Use case:** Automated release pipeline

## Setup Requirements

### Secrets
For the workflows to function properly, configure these GitHub secrets:

- `GITHUB_TOKEN` - Automatically provided by GitHub
- `CARGO_REGISTRY_TOKEN` - For publishing to crates.io (optional)
- `CODECOV_TOKEN` - For code coverage reports (optional)

### Branch Protection
Recommended branch protection rules for `main`:

1. Require status checks to pass before merging
2. Require branches to be up to date before merging
3. Required status checks:
   - `Test` (from CI workflow)
   - `Build` (from CI workflow)

## Local Development Commands

To run the same checks locally that CI runs:

```bash
# Format check (will fail CI if not formatted)
cargo fmt --all -- --check

# Auto-format code
cargo fmt --all

# Check compilation
cargo check --all-targets

# Clippy (strict mode like CI)
cargo clippy --all-targets -- -D warnings

# Run all tests
cargo test

# Security audit
cargo install cargo-audit
cargo audit
```

## Quick CI Test
Run this locally to verify CI will pass:

```bash
# Quick check equivalent
cargo fmt --all -- --check && \
cargo check --all-targets && \
cargo clippy --all-targets -- -W clippy::all && \
cargo test --bin doksnet
```

## Workflow Status

Monitor workflow status:
- 🟢 All green: Ready to merge/release
- 🟡 Running: Wait for completion
- 🔴 Failed: Check logs and fix issues

## Best Practices

1. **Always run `cargo fmt` before committing**
2. **Fix clippy warnings when possible**
3. **Add tests for new functionality**
4. **Keep the main branch stable**
5. **Use descriptive commit messages**
6. **Test locally before pushing**

## Troubleshooting

### Common CI Failures

**Formatting Issues:**
```bash
cargo fmt --all
git commit -am "Fix formatting"
```

**Clippy Warnings:**
```bash
cargo clippy --fix --all-targets
git commit -am "Fix clippy warnings"
```

**Test Failures:**
```bash
cargo test
# Fix failing tests, then commit
```

**Build Failures:**
```bash
cargo check --all-targets
# Fix compilation errors, then commit
```

### Platform-Specific Issues

If builds fail on specific platforms:
1. Check the workflow logs for the failing platform
2. Consider adding platform-specific dependencies or features
3. Test cross-compilation locally if possible

## Performance

Our CI is optimized for speed:
- **Caching:** Cargo dependencies cached between runs
- **Parallel jobs:** Different platforms build simultaneously  
- **Quick feedback:** Quick Check runs on every push
- **Comprehensive testing:** Full CI only on important branches

This setup provides fast feedback during development while ensuring comprehensive testing before releases. 