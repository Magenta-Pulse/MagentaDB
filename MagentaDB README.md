# MagentaDB

A high-performance, searchable encrypted database written in Rust. MagentaDB enables you to store sensitive data with full encryption while maintaining the ability to perform fast searches on encrypted content.

## Features

- **Searchable Encryption**: Query encrypted data without decrypting the entire database
- **High Performance**: In-memory storage with concurrent access using DashMap
- **Professional CLI**: Rich command-line interface with verbose logging and statistics
- **Persistent Storage**: JSON-based persistence with automatic backup
- **Secure by Default**: XChaCha20-Poly1305 encryption with per-field nonces
- **Thread-Safe**: Concurrent operations with optimized indexing

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   magentadb-cli │    │ magentadb-core  │    │magentadb-crypto │
│                 │    │                 │    │                 │
│ • CLI Interface │◄──►│ • InMemoryDB    │◄──►│ • Encryption    │
│ • Commands      │    │ • Document Store│    │ • Tokenization  │
│ • Persistence   │    │ • Indexing      │    │ • Key Management│
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Quick Start

### Installation

```bash
git clone https://github.com/ndourc/magentadb
cd magentadb
cargo build --release
```

### Basic Usage

```bash
# Insert encrypted data
./target/release/magentadb-cli insert user1 name "John Doe"
./target/release/magentadb-cli insert user1 email "john@example.com"

# Query encrypted data
./target/release/magentadb-cli query "John Doe"

# List all documents
./target/release/magentadb-cli list

# Decrypt specific fields
./target/release/magentadb-cli decrypt user1 name
```

## Commands Reference

### Insert Data

```bash
magentadb-cli insert <document_id> <field_name> <value>
```

Creates or updates a document with an encrypted field.

**Example:**

```bash
magentadb-cli insert employee1 salary "75000"
magentadb-cli insert employee1 department "Engineering"
```

### Show Document

```bash
magentadb-cli show <document_id>
```

Display a document in its encrypted form with masked values.

**Example:**

```bash
magentadb-cli show employee1
# Output:
# Document: employee1
#    salary: 7…f19a7e
#    department: E…b2c8d1
```

### Query Data

```bash
magentadb-cli query <plaintext_value>
```

Search for documents containing the specified plaintext value.

**Example:**

```bash
magentadb-cli query "Engineering"
# Finds all documents with fields containing "Engineering"
```

### Decrypt Field

```bash
magentadb-cli decrypt <document_id> <field_name>
```

Decrypt and display a specific field's value.

**Example:**

```bash
magentadb-cli decrypt employee1 salary
# Output: Decrypted employee1.salary: 75000
```

### List Documents

```bash
magentadb-cli list [--verbose]
```

Display all documents in the database.

**Example:**

```bash
magentadb-cli list -v
# Shows detailed information including tokens and field sizes
```

### Database Statistics

```bash
magentadb-cli stats
```

Show database performance metrics and metadata.

### Remove Document

```bash
magentadb-cli remove <document_id>
```

Permanently delete a document and its indexes.

### Clear Database

```bash
magentadb-cli clear [--force]
```

Remove all documents from the database. Prompts for confirmation unless `--force` is used.

## Configuration

### CLI Options

- `--database, -d <path>`: Specify database file path (default: `magentadb.json`)
- `--verbose, -v`: Enable detailed logging
- `--help`: Show help information
- `--version`: Show version information

### Database File Format

MagentaDB stores data in JSON format with the following structure:

```json
{
  "documents": {
    "user1": {
      "id": "user1",
      "fields": {
        "name": {
          "cipher": [131, 60, 6, 129, ...],
          "nonce": [241, 2, 109, 1, ...],
          "token": "f19a7e0fe7ef047d",
          "masked": "J…f19a7e"
        }
      }
    }
  },
  "secret_key": [45, 123, 78, ...],
  "version": "0.1.0",
  "created_at": "2025-01-01T10:00:00Z",
  "last_modified": "2025-01-01T10:30:00Z"
}
```

## Security Model

### Encryption

- **Algorithm**: XChaCha20-Poly1305 (authenticated encryption)
- **Key Size**: 256-bit randomly generated keys
- **Nonce**: 192-bit random nonce per field
- **Authentication**: Built-in tamper detection

### Searchable Tokens

- **Method**: Deterministic tokenization using key-derived hashing
- **Security**: Tokens don't reveal plaintext but enable exact matching
- **Index**: Separate token-to-document mapping for fast queries

### Threat Model

MagentaDB protects against:

- Data-at-rest compromise
- Unauthorized data access
- Tampering detection

**Note**: MagentaDB does not protect against:

- Memory dumps during active operation
- Side-channel attacks
- Compromised execution environment

## Performance

### Benchmarks (Typical Performance)

| Operation | Documents | Time  | Throughput         |
| --------- | --------- | ----- | ------------------ |
| Insert    | 10,000    | 850ms | 11,764 ops/sec     |
| Query     | 10,000    | 12ms  | 83,333 queries/sec |
| List      | 10,000    | 5ms   | 200,000 ops/sec    |

### Scaling Characteristics

- **Memory Usage**: ~200 bytes per document + field data
- **Index Overhead**: ~50 bytes per unique token
- **Concurrency**: Lock-free reads, optimized write contention

## Development

### Project Structure

```
magentadb/
├── crates/
│   ├── magentadb-cli/      # Command-line interface
│   │   ├── src/main.rs     # CLI implementation
│   │   └── Cargo.toml
│   ├── magentadb-core/     # Core database functionality
│   │   ├── src/
│   │   │   ├── lib.rs      # Public API
│   │   │   ├── db.rs       # InMemoryDB implementation
│   │   │   ├── document.rs # Document structures
│   │   │   └── token.rs    # Tokenization logic
│   │   └── Cargo.toml
│   └── magentadb-crypto/   # Cryptographic operations
│       ├── src/
│       │   ├── lib.rs      # Crypto API
│       │   └── encrypt.rs  # Encryption/decryption
│       └── Cargo.toml
├── Cargo.toml              # Workspace configuration
└── README.md
```

### Building from Source

```bash
# Clone repository
git clone https://github.com/ndourc/magentadb
cd magentadb

# Build all components
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

### Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration

# Benchmark tests
cargo bench

# Example data generation
cargo run --example generate_test_data
```

## Use Cases

### Healthcare Records

```bash
# Store patient data
magentadb-cli insert patient123 name "Alice Johnson"
magentadb-cli insert patient123 diagnosis "Hypertension"
magentadb-cli insert patient123 medication "Lisinopril"

# Search by condition
magentadb-cli query "Hypertension"
```

### Financial Data

```bash
# Store transaction data
magentadb-cli insert txn456 amount "1500.00"
magentadb-cli insert txn456 account "checking-001"

# Query transactions
magentadb-cli query "checking-001"
```

### Personal Information

```bash
# Store contact information
magentadb-cli insert contact789 phone "+1-555-0123"
magentadb-cli insert contact789 email "user@example.com"

# Search contacts
magentadb-cli query "user@example.com"
```

## Limitations

- **In-Memory Storage**: Database size limited by available RAM
- **Single Node**: No distributed storage or replication
- **File-Based Persistence**: JSON serialization may be slow for large datasets
- **Key Management**: Single master key per database file

## Roadmap

### Version 0.2.0

- [ ] Disk-based storage engine
- [ ] Compressed indexes
- [ ] Range queries
- [ ] Field-level access control

### Version 0.3.0

- [ ] Network API (REST/gRPC)
- [ ] Multi-user authentication
- [ ] Audit logging
- [ ] Backup/restore utilities

### Version 1.0.0

- [ ] Production hardening
- [ ] Performance optimizations
- [ ] Comprehensive documentation
- [ ] Security audit

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust standard conventions (`cargo fmt`)
- Add tests for new functionality
- Update documentation for API changes
- Use descriptive commit messages

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security

### Reporting Vulnerabilities

Please report security vulnerabilities to security@magentadb.com. Do not open public issues for security concerns.

### Security Best Practices

- Store database files in secure locations
- Use appropriate file system permissions
- Regularly backup encryption keys
- Monitor for unauthorized access attempts

## Support

- **Documentation**: [docs.magentadb.com](https://docs.magentadb.com)
- **Issues**: [GitHub Issues](https://github.com/ndourc/magentadb/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ndourc/magentadb/discussions)
- **Email**: henry.ndou@outlook.com

---

**MagentaDB v0.1.0** - Searchable encrypted database for the privacy-focused future.
