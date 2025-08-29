use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldMaterialized {
    pub cipher: Vec<u8>,
    pub nonce: Vec<u8>,
    pub token: String,
    pub masked: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentStored {
    pub id: String,
    pub fields: std::collections::HashMap<String, FieldMaterialized>,
}
