#[macro_use]
extern crate arrayref;

use grammers_crypto::aes::{ige_encrypt as _ige_encrypt, ige_decrypt as _ige_decrypt};
use wasm_bindgen::prelude::{wasm_bindgen};

#[wasm_bindgen]
pub fn ige_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    _ige_encrypt(plaintext, array_ref!(key, 0, 32), array_ref!(iv, 0, 32))
}

#[wasm_bindgen]
pub fn ige_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    _ige_decrypt(ciphertext, array_ref!(key, 0, 32), array_ref!(iv, 0, 32))
}