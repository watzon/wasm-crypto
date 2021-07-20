use std::iter::repeat;

#[macro_use]
extern crate arrayref;
use wasm_bindgen::{JsValue, throw_str};
use wasm_bindgen::prelude::{wasm_bindgen};

use grammers_crypto::aes::{ige_encrypt as _ige_encrypt, ige_decrypt as _ige_decrypt};
use crypto::aes::{ KeySize, ctr as _ctr };


// IGE

#[wasm_bindgen]
pub fn ige_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    if plaintext.len() % 16 != 0 {
        throw_str("plaintext must be divisible by 16")
    }

    if key.len() != 32 {
        throw_str("key must contain 32 bits")
    }

    if iv.len() != 32 {
        throw_str("iv must contain 32 bits")
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
        throw_str("iv must contain 32 bits")
    }

    Ok(_ige_decrypt(ciphertext, array_ref!(key, 0, 32), array_ref!(iv, 0, 32)))
}

// CTR

#[wasm_bindgen]
pub fn ctr128(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    if key.len() != 16 {
        throw_str("key must contain 16 bits")
    }

    if iv.len() != 16 {
        throw_str("iv must contain 16 bits")
    }

    let mut encrypter = _ctr(KeySize::KeySize128, &key[..], &iv[..]);
    let mut result: Vec<u8> = repeat(0).take(plaintext.len()).collect();
    encrypter.process(&plaintext[..], &mut result[..]);
    Ok(result)
}

#[wasm_bindgen]
pub fn ctr192(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    if key.len() != 24 {
        throw_str("key must contain 24 bits")
    }

    if iv.len() != 24 {
        throw_str("iv must contain 24 bits")
    }

    let mut encrypter = _ctr(KeySize::KeySize192, &key[..], &iv[..]);
    let mut result: Vec<u8> = repeat(0).take(plaintext.len()).collect();
    encrypter.process(&plaintext[..], &mut result[..]);
    Ok(result)
}

#[wasm_bindgen]
pub fn ctr256(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    if key.len() != 32 {
        throw_str("key must contain 32 bits")
    }

    if iv.len() != 32 {
        throw_str("iv must contain 32 bits")
    }

    let mut encrypter = _ctr(KeySize::KeySize256, &key[..], &iv[..]);
    let mut result: Vec<u8> = repeat(0).take(plaintext.len()).collect();
    encrypter.process(&plaintext[..], &mut result[..]);
    Ok(result)
}