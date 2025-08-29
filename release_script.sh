#!/bin/bash
# MagentaDB Release Preparation Script v0.1.0

set -e

VERSION="0.1.0-beta"
GITHUB_REPO="yourusername/magentadb"  # Update this

echo "Preparing MagentaDB v${VERSION} for release"
echo "==========================================="

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

# Check for uncommitted changes
if [[ -n $(git status --porcelain) ]]; then
    echo "Warning: Uncommitted changes detected"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Update version in Cargo.toml files
echo "Updating version numbers..."
find . -name "Cargo.toml" -exec sed -i.bak "s/^version = .*/version = \"${VERSION}\"/" {} \;

# Clean up backup files
find . -name "*.bak" -delete

# Run tests
echo "Running tests..."
cargo test --all

# Run clippy for code quality
echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
echo "Checking code formatting..."
cargo fmt --all -- --check

# Build release binaries
echo "Building release binaries..."
cargo build --release

# Run integration tests
echo "Running integration tests..."
chmod +x test_magentadb.sh
./test_magentadb.sh

# Generate documentation
echo "Generating documentation..."
cargo doc --all --no-deps

# Create changelog
echo "Creating CHANGELOG.md..."
cat > CHANGELOG.md << EOF
# Changelog

All notable changes to MagentaDB will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [${VERSION}] - $(date +%Y-%m-%d)

### Added
- Initial release of MagentaDB
- Searchable encryption using XChaCha20-Poly1305
- Thread-safe in-memory database with DashMap
- Professional CLI with rich formatting
- JSON-based persistence
- Document-field data model
- Token-based search indexing
- Database statistics and performance metrics
- Comprehensive documentation

### Security
- XChaCha20-Poly1305 authenticated encryption
- Per-field random nonces
- Deterministic tokenization for searches
- Secure key management

### Performance
- Lock-free concurrent reads
- Optimized write operations
- Efficient token indexing
- Memory-optimized data structures

## [Unreleased]

### Planned
- Disk-based storage engine
- REST API
- Multi-user support
- Range queries
- Compressed indexes

EOF

# Create security policy
echo "Creating SECURITY.md..."
cat > SECURITY.md << EOF
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Please report security vulnerabilities to security@magentadb.com or through GitHub's security advisory system.

**Do not open public issues for security vulnerabilities.**

### What to include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested fixes

We will respond within 48 hours and provide regular updates on the investigation.

## Security Best Practices

When using MagentaDB:

- Store database files in secure locations with appropriate permissions
- Regularly backup encryption keys
- Monitor database access logs
- Use strong, randomly generated passwords
- Keep software updated

## Threat Model

MagentaDB protects against:
- Unauthorized data access at rest
- Data tampering
- Plaintext exposure in storage

MagentaDB does not protect against:
- Memory dump attacks during active operation
- Side-channel attacks
- Compromised execution environments
- Social engineering attacks

EOF

# Prepare GitHub release assets
echo "Preparing release assets..."
mkdir -p release_assets

# Copy binary and documentation
cp target/release/magentadb-cli release_assets/
cp README.md release_assets/
cp LICENSE release_assets/
cp CHANGELOG.md release_assets/

# Create source tarball
git archive --format=tar.gz --prefix=magentadb-${VERSION}/ HEAD > release_assets/magentadb-${VERSION}-source.tar.gz

# Create checksums
cd release_assets
sha256sum * > checksums.txt
cd ..

# Commit version updates
git add -A
git commit -m "Release v${VERSION}"

# Create and push tag
git tag -a "v${VERSION}" -m "MagentaDB v${VERSION}

Initial release featuring:
- Searchable encryption with XChaCha20-Poly1305
- High-performance concurrent database
- Professional CLI interface
- JSON persistence
- Comprehensive documentation"

echo "Ready to push release..."
echo "Next steps:"
echo "1. Review changes: git log --oneline -10"
echo "2. Push to GitHub: git push origin main && git push origin v${VERSION}"
echo "3. GitHub Actions will automatically create the release"
echo "4. Upload additional assets from release_assets/ directory"

echo ""
echo "Release checklist:"
echo "- [x] Version numbers updated"
echo "- [x] Tests passing"
echo "- [x] Code quality checks passed"
echo "- [x] Documentation generated"
echo "- [x] Integration tests completed"
echo "- [x] Release assets prepared"
echo "- [x] Git tag created"
echo "- [ ] Push to GitHub"
echo "- [ ] Verify GitHub Actions build"
echo "- [ ] Update release description"
echo "- [ ] Announce release"

echo ""
echo "Release v${VERSION} is ready!"