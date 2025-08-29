// use hmac::{Hmac, Mac};
// use sha2::Sha256;
// use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

// type HmacSha256 = Hmac<Sha256>;

// pub fn tokenize(secret: &[u8], value: &str) -> String {
//     let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC key size");
//     mac.update(value.as_bytes());
//     let result = mac.finalize().into_bytes();
//     URL_SAFE_NO_PAD.encode(&result[..20])
// }
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a searchable token from plaintext using HMAC
pub fn tokenize(key: &[u8; 32], value: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(value.as_bytes());

    let result = mac.finalize();
    hex::encode(&result.into_bytes()[0..8])
}
