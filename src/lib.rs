#[macro_use]
extern crate arrayref;

use wasm_bindgen::{JsValue, throw_str};

use grammers_crypto::aes::{ige_encrypt as _ige_encrypt, ige_decrypt as _ige_decrypt};
use wasm_bindgen::prelude::{wasm_bindgen};

#[wasm_bindgen]
pub fn ige_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    if plaintext.len() % 16 != 0 {
        throw_str("plaintext must be divisible by 16")
    }

    if key.len() != 32 {
        throw_str("key must contain 32 bits")
    }

    if iv.len() != 32 {
        throw_str("key must contain 32 bits")
    }

    Ok(_ige_encrypt(plaintext, array_ref!(key, 0, 32), array_ref!(iv, 0, 32)))
}

#[wasm_bindgen]
pub fn ige_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    if ciphertext.len() % 16 != 0 {
        throw_str("ciphertext must be divisible by 16")
    }

    if key.len() != 32 {
        throw_str("key must contain 32 bits")
    }

    if iv.len() != 32 {
        throw_str("key must contain 32 bits")
    }

    Ok(_ige_decrypt(ciphertext, array_ref!(key, 0, 32), array_ref!(iv, 0, 32)))
}