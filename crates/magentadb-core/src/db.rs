use crate::document::DocumentStored;
use dashmap::DashMap;
use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub enum DBError {
    NotFound(String),
    StorageError(String),
    Duplicate(String),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::NotFound(id) => write!(f, "Document not found: {}", id),
            DBError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            DBError::Duplicate(id) => write!(f, "Duplicate document: {}", id),
        }
    }
}

impl std::error::Error for DBError {}

#[derive(Clone)]
pub struct InMemoryDB {
    documents: Arc<DashMap<String, Arc<DocumentStored>>>,
    token_index: Arc<DashMap<String, HashSet<String>>>,
    field_index: Arc<DashMap<String, HashSet<String>>>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        Self {
            documents: Arc::new(DashMap::new()),
            token_index: Arc::new(DashMap::new()),
            field_index: Arc::new(DashMap::new()),
        }
    }

    pub fn upsert(&self, doc: DocumentStored) -> Result<Option<Arc<DocumentStored>>, DBError> {
        let doc_id = doc.id.clone();
        let doc_arc = Arc::new(doc);

        if let Some(old_doc) = self.documents.get(&doc_id) {
            self.cleanup_indexes(&doc_id, &old_doc);
        }

        for (field_name, field_data) in &doc_arc.fields {
            self.token_index
                .entry(field_data.token.clone())
                .or_insert_with(HashSet::new)
                .insert(doc_id.clone());

            self.field_index
                .entry(field_name.clone())
                .or_insert_with(HashSet::new)
                .insert(doc_id.clone());
        }

        let old_doc = self.documents.insert(doc_id, doc_arc);
        Ok(old_doc)
    }

    pub fn get(&self, id: &str) -> Result<Arc<DocumentStored>, DBError> {
        self.documents
            .get(id)
            .map(|entry| Arc::clone(&entry))
            .ok_or_else(|| DBError::NotFound(id.to_string()))
    }

    pub fn query_by_token(&self, token: &str) -> Vec<Arc<DocumentStored>> {
        let doc_ids = match self.token_index.get(token) {
            Some(ids) => ids.clone(),
            None => return Vec::new(),
        };

        let mut results = Vec::with_capacity(doc_ids.len());
        for id in doc_ids {
            if let Some(doc) = self.documents.get(&id) {
                results.push(Arc::clone(&doc));
            }
        }
        results
    }

    pub fn remove(&self, id: &str) -> Result<Arc<DocumentStored>, DBError> {
        if let Some((_key, doc)) = self.documents.remove(id) {
            self.cleanup_indexes(id, &doc);
            Ok(doc)
        } else {
            Err(DBError::NotFound(id.to_string()))
        }
    }

    pub fn clear(&self) {
        self.documents.clear();
        self.token_index.clear();
        self.field_index.clear();
    }

    pub fn all_ids(&self) -> Vec<String> {
        self.documents
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }

    pub fn stats(&self) -> DBStats {
        DBStats {
            document_count: self.documents.len(),
            token_index_size: self.token_index.len(),
            field_index_size: self.field_index.len(),
        }
    }

    fn cleanup_indexes(&self, doc_id: &str, doc: &DocumentStored) {
        for (field_name, field_data) in &doc.fields {
            if let Some(mut token_ids) = self.token_index.get_mut(&field_data.token) {
                token_ids.remove(doc_id);
                if token_ids.is_empty() {
                    drop(token_ids);
                    self.token_index.remove(&field_data.token);
                }
            }

            if let Some(mut field_ids) = self.field_index.get_mut(field_name) {
                field_ids.remove(doc_id);
                if field_ids.is_empty() {
                    drop(field_ids);
                    self.field_index.remove(field_name);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DBStats {
    pub document_count: usize,
    pub token_index_size: usize,
    pub field_index_size: usize,
}
