use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use magentadb_core::{
    db::InMemoryDB,
    document::{DocumentStored, FieldMaterialized},
};
use magentadb_crypto::{decrypt, encrypt, token};

#[derive(Parser)]
#[command(name = "magentadb")]
#[command(about = "A searchable encrypted database")]
#[command(version = "0.1.0")]
struct Cli {
    /// Database file path
    #[arg(short, long, default_value = "magentadb.json")]
    database: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Insert a new document or update an existing one
    Insert {
        /// Document ID
        id: String,
        /// Field name
        field: String,
        /// Field value to encrypt
        value: String,
    },

    /// Show a document by ID (encrypted form)
    Show {
        /// Document ID
        id: String,
    },

    /// Query documents by plaintext value
    Query {
        /// Value to search for
        value: String,
    },

    /// Decrypt a specific field in a document
    Decrypt {
        /// Document ID
        id: String,
        /// Field name to decrypt
        field: String,
    },

    /// List all documents in the database
    List,

    /// Show database statistics
    Stats,

    /// Remove a document by ID
    Remove {
        /// Document ID
        id: String,
    },

    /// Clear the entire database
    Clear {
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}

/// Database state for persistence
#[derive(Serialize, Deserialize)]
struct DatabaseState {
    documents: HashMap<String, DocumentStored>,
    secret_key: [u8; 32],
    version: String,
    created_at: String,
    last_modified: String,
}

impl DatabaseState {
    fn load_or_create(path: &str) -> Result<Self> {
        if let Ok(data) = fs::read_to_string(path) {
            let mut state: DatabaseState =
                serde_json::from_str(&data).context("Failed to parse database file")?;

            // Update last accessed time
            state.last_modified = chrono::Utc::now().to_rfc3339();

            println!("✓ Loaded existing database from {}", path);
            println!(
                "  └─ {} documents, created {}",
                state.documents.len(),
                state.created_at
            );

            Ok(state)
        } else {
            println!("📄 Creating new database at {}", path);
            let now = chrono::Utc::now().to_rfc3339();

            Ok(Self {
                documents: HashMap::new(),
                secret_key: rand::thread_rng().gen(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                created_at: now.clone(),
                last_modified: now,
            })
        }
    }

    fn save(&mut self, path: &str) -> Result<()> {
        self.last_modified = chrono::Utc::now().to_rfc3339();

        let data = serde_json::to_string_pretty(self).context("Failed to serialize database")?;

        fs::write(path, data).context("Failed to write database file")?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("🔧 MagentaDB v{}", env!("CARGO_PKG_VERSION"));
        println!("📂 Database: {}", cli.database);
    }

    let mut db_state = DatabaseState::load_or_create(&cli.database)?;
    let db = InMemoryDB::new();

    // Load existing documents into the in-memory DB
    for doc in db_state.documents.values() {
        db.upsert(doc.clone())
            .context(format!("Failed to load document {}", doc.id))?;
    }

    let result = match &cli.command {
        Commands::Insert { id, field, value } => handle_insert(
            &db,
            &mut db_state,
            id,
            field,
            value,
            &cli.database,
            cli.verbose,
        ),

        Commands::Show { id } => handle_show(&db, id, cli.verbose),

        Commands::Query { value } => handle_query(&db, &db_state, value, cli.verbose),

        Commands::Decrypt { id, field } => handle_decrypt(&db, &db_state, id, field),

        Commands::List => handle_list(&db, cli.verbose),

        Commands::Stats => handle_stats(&db, &db_state),

        Commands::Remove { id } => handle_remove(&db, &mut db_state, id, &cli.database),

        Commands::Clear { force } => handle_clear(&db, &mut db_state, &cli.database, *force),
    };

    if let Err(e) = result {
        eprintln!(" Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn handle_insert(
    db: &InMemoryDB,
    db_state: &mut DatabaseState,
    id: &str,
    field: &str,
    value: &str,
    db_path: &str,
    verbose: bool,
) -> Result<()> {
    let (nonce, cipher) = encrypt(value.as_bytes(), &db_state.secret_key);
    let tok = token::tokenize(&db_state.secret_key, value);

    let masked = if value.len() >= 2 && tok.len() >= 6 {
        format!("{}…{}", &value.chars().next().unwrap(), &tok[0..6])
    } else if !value.is_empty() {
        format!("{}…", &value.chars().next().unwrap())
    } else {
        "…".to_string()
    };

    // Check if document exists and merge fields
    let mut fields = if let Ok(existing_doc) = db.get(id) {
        existing_doc
            .fields
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    } else {
        HashMap::new()
    };

    fields.insert(
        field.to_string(),
        FieldMaterialized {
            cipher,
            nonce,
            token: tok.clone(),
            masked: masked.clone(),
        },
    );

    let doc = DocumentStored {
        id: id.to_string(),
        fields,
    };

    db.upsert(doc.clone())?;
    db_state.documents.insert(id.to_string(), doc);
    db_state.save(db_path)?;

    if verbose {
        println!("📝 Inserted field '{}' in document '{}'", field, id);
        println!("   └─ Token: {}, Masked: {}", tok, masked);
    } else {
        println!("✓ Inserted document '{}'", id);
    }

    Ok(())
}

fn handle_show(db: &InMemoryDB, id: &str, verbose: bool) -> Result<()> {
    match db.get(id) {
        Ok(doc) => {
            println!("📄 Document: {}", id);
            for (field_name, field_data) in &doc.fields {
                println!("   {}: {}", field_name, field_data.masked);
                if verbose {
                    println!("     └─ Token: {}", field_data.token);
                    println!("     └─ Cipher size: {} bytes", field_data.cipher.len());
                    println!("     └─ Nonce size: {} bytes", field_data.nonce.len());
                }
            }
            Ok(())
        }
        Err(_) => {
            println!(" Document '{}' not found", id);
            Ok(())
        }
    }
}

fn handle_query(
    db: &InMemoryDB,
    db_state: &DatabaseState,
    value: &str,
    verbose: bool,
) -> Result<()> {
    let tok = token::tokenize(&db_state.secret_key, value);
    let results = db.query_by_token(&tok);

    if results.is_empty() {
        println!("🔍 No documents found matching '{}'", value);
        if verbose {
            println!("   └─ Search token: {}", tok);
        }
    } else {
        println!(
            "🔍 Found {} document(s) matching '{}':",
            results.len(),
            value
        );
        for doc in results {
            println!("   📄 {}", doc.id);
            for (field_name, field_data) in &doc.fields {
                if field_data.token == tok {
                    println!("      └─ {}: {}", field_name, field_data.masked);
                }
            }
        }
    }

    Ok(())
}

fn handle_decrypt(db: &InMemoryDB, db_state: &DatabaseState, id: &str, field: &str) -> Result<()> {
    let doc = db.get(id).context(format!("Document '{}' not found", id))?;

    let field_data = doc
        .fields
        .get(field)
        .context(format!("Field '{}' not found in document '{}'", field, id))?;

    let plaintext = decrypt(&field_data.cipher, &field_data.nonce, &db_state.secret_key)
        .context("Failed to decrypt field")?;

    let text = String::from_utf8(plaintext).context("Decrypted data is not valid UTF-8")?;

    println!("🔓 Decrypted {}.{}: {}", id, field, text);

    Ok(())
}

fn handle_list(db: &InMemoryDB, verbose: bool) -> Result<()> {
    let all_docs = db.all_ids();

    if all_docs.is_empty() {
        println!("📭 No documents in database");
        return Ok(());
    }

    println!("📋 Database contains {} document(s):", all_docs.len());

    for doc_id in all_docs {
        let doc = db.get(&doc_id)?;
        let field_count = doc.fields.len();
        let field_names: Vec<String> = doc.fields.keys().cloned().collect();

        println!(
            "   📄 {} ({} field{})",
            doc_id,
            field_count,
            if field_count == 1 { "" } else { "s" }
        );

        if verbose {
            for (field_name, field_data) in &doc.fields {
                println!(
                    "      └─ {}: {} [{}]",
                    field_name, field_data.masked, field_data.token
                );
            }
        } else {
            println!("      └─ Fields: [{}]", field_names.join(", "));
        }
    }

    Ok(())
}

fn handle_stats(db: &InMemoryDB, db_state: &DatabaseState) -> Result<()> {
    let stats = db.stats();

    println!(" Database Statistics:");
    println!("   Documents: {}", stats.document_count);
    println!("   Token index size: {}", stats.token_index_size);
    println!("   Field index size: {}", stats.field_index_size);
    println!("   Version: {}", db_state.version);
    println!("   Created: {}", db_state.created_at);
    println!("   Last modified: {}", db_state.last_modified);

    // Calculate total fields
    let total_fields: usize = db_state
        .documents
        .values()
        .map(|doc| doc.fields.len())
        .sum();
    println!("   Total fields: {}", total_fields);

    Ok(())
}

fn handle_remove(
    db: &InMemoryDB,
    db_state: &mut DatabaseState,
    id: &str,
    db_path: &str,
) -> Result<()> {
    match db.remove(id) {
        Ok(_) => {
            db_state.documents.remove(id);
            db_state.save(db_path)?;
            println!("  Removed document '{}'", id);
            Ok(())
        }
        Err(_) => {
            println!(" Document '{}' not found", id);
            Ok(())
        }
    }
}

fn handle_clear(
    db: &InMemoryDB,
    db_state: &mut DatabaseState,
    db_path: &str,
    force: bool,
) -> Result<()> {
    if !force {
        print!("⚠️  This will delete all documents. Are you sure? (y/N): ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let doc_count = db_state.documents.len();

    db.clear();
    db_state.documents.clear();
    db_state.save(db_path)?;

    println!("🧹 Cleared database ({} documents removed)", doc_count);

    Ok(())
}
