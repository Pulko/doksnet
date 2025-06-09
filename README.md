# 🦀 doksnet

[![CI](https://github.com/username/doksnet/workflows/CI/badge.svg)](https://github.com/Pulko/doksnet/actions/workflows/ci.yml)
[![Quick Check](https://github.com/username/doksnet/workflows/Quick%20Check/badge.svg)](https://github.com/Pulko/doksnet/actions/workflows/quick-check.yml)
[![Crates.io](https://img.shields.io/crates/v/doksnet.svg)](https://crates.io/crates/doksnet)
[![Documentation](https://docs.rs/doksnet/badge.svg)](https://docs.rs/doksnet)

A Rust CLI tool for **documentation ↔ code mapping verification**. Create lightweight symbolic links between documentation sections and code snippets, then verify that both sides stay synchronized using cryptographic hashes.

## 🚀 Installation

```bash
# Clone and build
git clone <repository>
cd doksnet
cargo build --release

# Install globally (optional)
cargo install --path .
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
# .doks v2 - Compact format
version=0.2.0
default_doc=README.md

# Format: id|doc_partition|code_partition|doc_hash|code_hash|description
a1b2c3d4|README.md:10-15|src/lib.rs:20-35|abc123def456...|789xyz012abc...|API usage example
main-func|README.md:25-30|src/main.rs:1-10|fedcba987654...|123456789abc...|Main function example
```

**Benefits of the compact format:**
- 📦 **5x smaller** than TOML (faster parsing, less storage)
- ⚡ **Machine-optimized** (perfect for automation)
- 🔧 **Grep-friendly** (easy to analyze with standard tools)
- 🚀 **Simple parsing** (no complex dependencies)

## 🔄 Typical Workflow

```bash
# 1. Initialize project
doksnet new

# 2. Create mappings between docs and code
doksnet add   # Repeat as needed

# 3. In CI/CD pipeline
doksnet test  # Fails build if docs/code drift apart

# 4. When changes are detected
doksnet test-interactive  # Guided fixing

# 5. Edit specific mappings
doksnet edit a1b2c3d4

# 6. Clean up broken mappings
doksnet remove-failed
```

## 🎯 Use Cases

- **API Documentation**: Link examples in README to actual implementation
- **Tutorial Sync**: Ensure code samples in guides match working code
- **Architecture Docs**: Connect design decisions to code structures
- **Code Reviews**: Verify documentation updates accompany code changes
- **Legacy Code**: Track which docs describe which code sections

## 🚧 Future Extensions

- **CI/CD Integration**: Pre-built GitHub Actions
- **VSCode Extension**: GUI for creating/managing mappings
- **Diff Visualization**: Show exact changes between versions
- **Batch Operations**: Mass edit/update operations
- **Export Formats**: Generate reports in various formats

---

**Built with ❤️ in Rust** • Lightweight • Fast • Reliable 