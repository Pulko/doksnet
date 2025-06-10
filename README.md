
<img width="881" alt="Screenshot 2025-06-10 at 15 35 24" src="https://github.com/user-attachments/assets/7a20508c-8640-4a51-b371-bf5692eaea7c" />

# 🦀 doksnet

[![CI](https://github.com/Pulko/doksnet/workflows/CI/badge.svg)](https://github.com/Pulko/doksnet/actions/workflows/ci.yml)
[![Quick Check](https://github.com/Pulko/doksnet/workflows/Quick%20Check/badge.svg)](https://github.com/Pulko/doksnet/actions/workflows/quick-check.yml)
[![Crates.io](https://img.shields.io/crates/v/doksnet.svg)](https://crates.io/crates/doksnet)
[![Documentation](https://docs.rs/doksnet/badge.svg)](https://docs.rs/doksnet)

A Rust CLI tool for **documentation ↔ code mapping verification**. Create lightweight symbolic links between documentation sections and code snippets, then verify that both sides stay synchronized using cryptographic hashes.

## 🚀 Installation

### From crates.io (Recommended)

```bash
# Install from crates.io
cargo install doksnet
```

### From GitHub Releases

Download pre-built binaries from [GitHub Releases](https://github.com/Pulko/doksnet/releases):

- **Linux**: `doksnet-linux-amd64`
- **Windows**: `doksnet-windows-amd64.exe`  
- **macOS**: `doksnet-macos-amd64` (Intel) or `doksnet-macos-arm64` (Apple Silicon)

### From Source (Development)

```bash
# Clone and build from source
git clone https://github.com/Pulko/doksnet
cd doksnet
cargo install --path .
```

### From GitHub Pipeline (CI/CD)

```yaml
  - uses: Pulko/doksnet@v1
```

## 📋 Commands Overview

| Command | Purpose | Interactive | CI/CD Safe |
|---------|---------|-------------|------------|
| `new` | Initialize a `.doks` file | ✅ | ❌ |
| `add` | Create doc↔code mappings | ✅ | ❌ |
| `edit <id>` | Edit specific mapping | ✅ | ❌ |
| `remove-failed` | Remove all failed mappings | ✅ | ❌ |
| `test` | Verify all mappings | ❌ | ✅ |
| `test-interactive` | Test with guided fixing | ✅ | ❌ |

## 🛠 Usage Guide

### 1. Initialize Project

```bash
# Create .doks file in current directory
doksnet new

# Create .doks file in specific directory  
doksnet new /path/to/project
```

**What it does:**
- Scans for documentation files (README.md, etc.)
- Prompts you to select default documentation file
- Creates `.doks` configuration file

### 2. Create Documentation-Code Mappings

```bash
doksnet add
```

**Interactive flow:**
1. **Documentation partition**: `README.md:15-25` or `README.md:10-20@5-30`
2. **Content preview**: Shows extracted documentation text
3. **Confirmation**: Verify the selection is correct
4. **Code partition**: `src/lib.rs:45-60` or `src/main.rs:10-25@10-50`
5. **Content preview**: Shows extracted code text
6. **Confirmation**: Verify the selection is correct
7. **Description**: Optional description for the mapping
8. **Hash generation**: Creates Blake3 hashes and saves mapping

### 3. Edit Existing Mappings

```bash
# Edit by ID (first 8 characters sufficient)
doksnet edit a1b2c3d4
```

**What you can edit:**
- Documentation partition reference
- Code partition reference  
- Description
- Both partitions at once

**Features:**
- Shows current values
- Pre-fills input with existing values
- Previews new content before applying
- Updates hashes automatically

### 4. Test Mappings (CI/CD)

```bash
# Non-interactive testing for automation
doksnet test
```

**Output:**
- ✅ **PASS**: Content matches stored hashes
- ❌ **FAIL**: Content has changed
- **Exit code 1** if any mappings fail (perfect for CI/CD)

### 5. Interactive Testing & Fixing

```bash
# Interactive mode with change preview
doksnet test-interactive
```

**For each failed mapping, you can:**
- **Update hashes**: Accept current content as new baseline
- **Edit mapping**: Redirect to `doksnet edit <id>`
- **Remove mapping**: Delete the broken mapping
- **Skip**: Leave as-is for now

**Shows:**
- Current content that changed
- Hash mismatches
- Detailed change previews

### 6. Bulk Remove Failed Mappings

```bash
# Remove all mappings that fail verification
doksnet remove-failed
```

**Safety features:**
- Lists all failed mappings before removal
- Shows failure reasons (doc/code/both)
- Requires confirmation before deletion

## 📄 Partition Format

Partitions use this lightweight format to reference file ranges:

```
<relative_path>:<start_line>-<end_line>@<start_col>-<end_col>
```

**Examples:**
- `README.md` - Entire file
- `README.md:10-20` - Lines 10-20
- `README.md:15` - Single line 15
- `src/lib.rs:10-20@5-30` - Lines 10-20, columns 5-30
- `docs/guide.md:1-5@1-50` - First 5 lines, first 50 characters

**Notes:**
- Line numbers are **1-indexed**
- Column numbers are **1-indexed**  
- Ranges are **inclusive**
- Non-contiguous ranges require multiple mappings

## 🔐 Hash-Based Verification

**How it works:**
1. **Text → Hash**: Extract content from partition → Generate Blake3 hash
2. **Hash ← Text**: Re-extract content → Compare with stored hash
3. **Change Detection**: Any character-level change produces different hash

**What's detected:**
- Content changes
- Whitespace modifications
- Line ending changes
- File deletions/moves
- Invalid partition ranges

## 📁 .doks File Structure

The `.doks` file uses a compact, machine-optimized format:

```
# .doks - Mapping doks to code 
default_doc=README.md

# Format: id|doc_partition|code_partition|doc_hash|code_hash|description
d9639aad-b4c9-4e47-94a4-ef6a1ad25f63|README.md:18@3-33|src/app/page.tsx:95|23bb378a2db6d108b38097af5901d3360452fbdd3593fb5a31cfc3974b76c6b1|2864eeedc71061f92ae67be5a7b1617107b70a1fd0aa4515b985df0ce2f6ec9c|Deploying application on Vercel
d0766781-70f9-45a4-a866-f999cd2d040d|README.md:8|src/app/create/page.tsx:360-902|4362febe3c26828db16c3ed873fdd57909da39866371652ae68310ee4648bc38|f870ce178ec7c66cb7e109510eb6b0637a31a73505221d8ebf147a252545fe88|Form for portfolio creating
071e85a5-4557-4c1f-aeaf-319a1d0f69c2|README.md:53|package.json:6|cc9fd37fa5cca2219e39826ddaf515b337d376becf2c207b9b5293b51e42b7c3|6235c68106534c519d0b3b0a8d2cd38bc139ef2580efd04a47f427ec2b47fd7c|Start the application

```

**Benefits of the compact format:**
- 📦 **5x smaller** than TOML (faster parsing, less storage)
- ⚡ **Machine-optimized** (perfect for automation)
- 🔧 **Grep-friendly** (easy to analyze with standard tools)
- 🚀 **Simple parsing** (no complex dependencies)

## 🔄 Typical Workflow

### Local Development

```bash
# 1. Initialize project
doksnet new

# 2. Create mappings between docs and code
doksnet add   # Repeat as needed

# 3. Test locally
doksnet test

# 4. When changes are detected
doksnet test-interactive  # Guided fixing

# 5. Edit specific mappings
doksnet edit a1b2c3d4

# 6. Clean up broken mappings
doksnet remove-failed
```

### CI/CD Integration

**Using GitHub Action (Recommended):**

```yaml
# .github/workflows/docs.yml
name: Documentation Sync

on: [push, pull_request]

jobs:
  verify-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Pulko/doksnet@v1
```

## 🎯 Use Cases

- **API Documentation**: Link examples in README to actual implementation
- **Tutorial Sync**: Ensure code samples in guides match working code
- **Architecture Docs**: Connect design decisions to code structures
- **Code Reviews**: Verify documentation updates accompany code changes
- **Legacy Code**: Track which docs describe which code sections

## 🚀 GitHub Action

Doksnet provides a ready-to-use GitHub Action for seamless CI/CD integration:

### Basic Usage

```yaml
name: Documentation Sync Check
on: [push, pull_request]

jobs:
  verify-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Pulko/doksnet@v1
        # Uses defaults: command='test', fail-on-error=true
```

### Advanced Configuration

```yaml
- uses: Pulko/doksnet@v1
  with:
    command: 'test'           # Command to run
    version: 'latest'         # Doksnet version  
    working-directory: '.'    # Directory to run in
    fail-on-error: true       # Fail workflow on issues
```

### Action Inputs

| Input | Description | Default | Options |
|-------|-------------|---------|---------|
| `command` | Doksnet command to run | `test` | `test`, `remove-failed` |
| `version` | Doksnet version to use | `latest` | `latest`, `0.2.0`, etc. |
| `working-directory` | Directory to run doksnet in | `.` | Any valid path |
| `fail-on-error` | Fail workflow if issues found | `true` | `true`, `false` |

### Action Outputs

| Output | Description |
|--------|-------------|
| `result` | Full output from doksnet command |
| `exit-code` | Exit code (0 = success, 1 = failure) |

### Common Patterns

**Enforce Documentation Sync (Fail Build):**
```yaml
- uses: Pulko/doksnet@v1
  # Fails CI if docs are out of sync (default behavior)
```

**Warning Only (Don't Fail Build):**
```yaml
- uses: Pulko/doksnet@v1
  with:
    fail-on-error: false
```

**Cleanup Failed Mappings:**
```yaml
- uses: Pulko/doksnet@v1
  with:
    command: 'remove-failed'
```

## 🚧 Future Extensions

- **VSCode Extension**: GUI for creating/managing mappings
- **Diff Visualization**: Show exact changes between versions
- **Batch Operations**: Mass edit/update operations
- **Export Formats**: Generate reports in various formats

---

**Built with ❤️ in Rust** • Lightweight • Fast • Reliable 
