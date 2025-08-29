# GitHub Setup and Release Guide for MagentaDB

## Initial Repository Setup

### 1. Create GitHub Repository

1. Go to [GitHub](https://github.com) and click "New repository"
2. Repository name: `magentadb`
3. Description: "A high-performance, searchable encrypted database written in Rust"
4. Set as Public
5. Don't initialize with README (we have our own)
6. Click "Create repository"

### 2. Local Repository Setup

```bash
# Initialize git repository (if not already done)
cd /path/to/your/magentadb
git init

# Add all files
git add .

# Initial commit
git commit -m "Initial commit: MagentaDB v0.1.0-beta

- Searchable encryption with XChaCha20-Poly1305
- High-performance concurrent database using DashMap
- Professional CLI with comprehensive commands
- JSON-based persistence
- Complete documentation and examples"

# Add remote origin (replace with your GitHub username)
git remote add origin https://github.com/YOURUSERNAME/magentadb.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### 3. Repository Structure Check

Ensure your repository has this structure:

```
magentadb/
├── .github/
│   └── workflows/
│       └── release.yml
├── crates/
│   ├── magentadb-cli/
│   │   ├── src/
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   ├── magentadb-core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── db.rs
│   │   │   ├── document.rs
│   │   │   └── token.rs
│   │   └── Cargo.toml
│   └── magentadb-crypto/
│       ├── src/
│       │   ├── lib.rs
│       │   └── encrypt.rs
│       └── Cargo.toml
├── Cargo.toml
├── README.md
├── LICENSE
├── CHANGELOG.md
├── SECURITY.md
├── test_magentadb.sh
└── release.sh
```

## Pre-Release Testing

### 1. Run Local Tests

```bash
# Build and test
cargo build --release
cargo test --all
cargo clippy --all-targets --all-features

# Run integration tests
chmod +x test_magentadb.sh
./test_magentadb.sh
```

### 2. Test CLI Functionality

```bash
CLI="./target/release/magentadb-cli"

# Test basic operations
$CLI insert test1 name "John Doe"
$CLI insert test1 email "john@example.com"
$CLI list -v
$CLI query "John Doe"
$CLI decrypt test1 name
$CLI stats
$CLI clear --force
```

## GitHub Release Process

### 1. Prepare Release Branch

```bash
# Create release branch
git checkout -b release/v0.1.0-beta

# Run release preparation script
chmod +x release.sh
./release.sh

# Review changes
git log --oneline -5
git diff HEAD~1
```

### 2. Push to GitHub

```bash
# Push branch and tag
git push origin release/v0.1.0-beta
git push origin v0.1.0-beta
```

### 3. Monitor GitHub Actions

1. Go to your repository on GitHub
2. Click "Actions" tab
3. Watch the "Release" workflow
4. Verify builds complete for all platforms:
   - Linux (x86_64)
   - Windows (x86_64)
   - macOS (x86_64 and Apple Silicon)

### 4. Customize Release

1. Go to "Releases" in your GitHub repository
2. Click on the auto-created release
3. Click "Edit release"
4. Update the description with specific details:

```markdown
## MagentaDB v0.1.0-beta 🚀

### What is MagentaDB?

MagentaDB is a high-performance, searchable encrypted database that allows you to store sensitive data with full encryption while maintaining fast search capabilities.

### Key Features

- **Searchable Encryption**: Query encrypted data without decrypting the entire database
- **High Performance**: Concurrent operations with lock-free reads
- **Professional CLI**: Rich command-line interface with detailed output
- **Secure by Default**: XChaCha20-Poly1305 encryption with random nonces
- **Easy to Use**: Simple commands for complex operations

### Installation

Download the binary for your platform:

- **Linux**: `magentadb-cli-linux-x86_64`
- **Windows**: `magentadb-cli-windows-x86_64.exe`
- **macOS (Intel)**: `magentadb-cli-macos-x86_64`
- **macOS (Apple Silicon)**: `magentadb-cli-macos-aarch64`

Make it executable and run:

```bash
# Linux/macOS
chmod +x magentadb-cli-*
./magentadb-cli-* --help

# Windows
magentadb-cli-windows-x86_64.exe --help
```

### Quick Start

```bash
# Insert encrypted data
magentadb-cli insert user1 name "Alice Johnson"
magentadb-cli insert user1 email "alice@example.com"

# Search encrypted data
magentadb-cli query "Alice Johnson"

# List all documents
magentadb-cli list

# Show database stats
magentadb-cli stats
```

### What's New in v0.1.0-beta

This is the initial beta release of MagentaDB with core functionality:

- ✅ Document storage with field-level encryption
- ✅ Token-based searchable encryption
- ✅ Thread-safe concurrent operations
- ✅ JSON persistence
- ✅ Professional CLI interface
- ✅ Database statistics and management
- ✅ Comprehensive documentation

### Known Limitations

- In-memory storage only (persistent to disk as JSON)
- Single database per file
- No network API (CLI only)
- Limited to exact-match queries

### Feedback Welcome

This is a beta release. Please report issues, suggestions, and feedback:

- **Issues**: [GitHub Issues](https://github.com/YOURUSERNAME/magentadb/issues)
- **Discussions**: [GitHub Discussions](https://github.com/YOURUSERNAME/magentadb/discussions)

### Checksums

```
[Checksums will be auto-generated by the release process]
```
```

## Post-Release Activities

### 1. Verify Release Assets

1. Download each binary from the release page
2. Test on different platforms if possible
3. Verify checksums match

### 2. Update Documentation

```bash
# Update README with release information
# Update any version references
# Add usage examples with the new release
```

### 3. Announce the Release

Consider announcing on:

- Social media (Twitter/X, LinkedIn)
- Rust community forums
- Hacker News (if appropriate)
- Reddit (r/rust, r/programming)
- Your blog/website

### 4. Merge Release Branch

```bash
git checkout main
git merge release/v0.1.0-beta
git push origin main
```

## Future Release Process

For subsequent releases:

1. Create new release branch: `git checkout -b release/v0.2.0`
2. Update version numbers
3. Update CHANGELOG.md
4. Run tests and quality checks
5. Create tag and push
6. GitHub Actions will handle the rest

## Troubleshooting

### Common Issues

**Build Failures:**
- Check Rust version compatibility
- Verify all dependencies are available
- Review error logs in GitHub Actions

**Missing Assets:**
- Ensure GitHub Actions completed successfully
- Check upload permissions
- Verify file paths in workflow

**Permission Issues:**
- Make sure binaries are executable: `chmod +x filename`
- Check file system permissions

### Getting Help

- Check GitHub Actions logs
- Review Cargo.toml dependencies
- Test locally before pushing
- Ask in GitHub Discussions

---

Ready to release MagentaDB v0.1.0-beta to the world!