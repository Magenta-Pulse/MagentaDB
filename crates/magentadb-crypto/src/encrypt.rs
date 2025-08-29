use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};

pub fn encrypt(plaintext: &[u8], key_bytes: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let key = Key::from(*key_bytes);
    let cipher = XChaCha20Poly1305::new(&key);
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut nonce);
    let ct = cipher
        .encrypt(&XNonce::from(nonce), plaintext)
        .expect("encryption failed");
    (nonce.to_vec(), ct)
}

pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key_bytes: &[u8; 32]) -> Result<Vec<u8>> {
    let key = Key::from(*key_bytes);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce_array: [u8; 24] = nonce
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid nonce length"))?;

    cipher
        .decrypt(&XNonce::from(nonce_array), ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
}
