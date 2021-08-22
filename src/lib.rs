use std::iter::repeat;

#[macro_use]
extern crate arrayref;
use js_sys::{Uint8Array};
use wasm_bindgen::{JsValue, throw_str};
use wasm_bindgen::prelude::{wasm_bindgen};

use grammers_crypto::aes::{ige_encrypt as _ige_encrypt, ige_decrypt as _ige_decrypt};
use grammers_crypto::auth_key::{AuthKey as _AuthKey};
use grammers_crypto::factorize::{factorize as _factorize};
use grammers_crypto::rsa::{Key as _Key, encrypt_hashed as _encrypt_hashed};
use crypto::aes::{ KeySize, ctr as _ctr };
use crc32fast::Hasher;


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

// RSA
#[wasm_bindgen]
pub struct AuthKey {
    auth_key: _AuthKey
}

#[wasm_bindgen]
impl AuthKey {
    pub fn from_bytes(data: &[u8]) -> Self {
        if data.len() != 256 {
            throw_str("data must contain 256 bytes")
        }

        let key = _AuthKey::from_bytes(array_ref!(data, 0, 256).to_owned());
        Self {
            auth_key: key
        }
    }

    pub fn to_bytes(&self) -> Uint8Array {
        let bytes = self.auth_key.to_bytes();
        Uint8Array::from(&bytes[..])
    }

    pub fn calc_new_nonce_hash(&self, new_nonce: &[u8], number: u8) -> Uint8Array {
        if new_nonce.len() != 32 {
            throw_str("data must contain 32 bytes")
        }

        let hash = self.auth_key.calc_new_nonce_hash(array_ref!(new_nonce, 0, 32), number);
        Uint8Array::from(&hash[..])
    }
}

#[wasm_bindgen]
pub fn factorize(pq: u64) -> Vec<u64> {
    let res = _factorize(pq);
    let mut vec: Vec<u64> = Vec::new();
    vec.push(res.0);
    vec.push(res.1);
    vec.into_iter().collect()
}

#[wasm_bindgen]
pub struct RsaKey {
    key: _Key
}

#[wasm_bindgen]
impl RsaKey {
    pub fn new(n: &str, e: &str) -> Option<RsaKey> {
        let key = _Key::new(n, e);
        match key {
            Some(k) => Some(Self { key: k }),
            None => None
        }
    }

    pub fn encrypt_hashed(&self, data: &[u8], random_bytes: &[u8]) -> Vec<u8> {
        if random_bytes.len() != 256 {
            throw_str("random_bytes must contain 256 bytes exactly")
        }

        _encrypt_hashed(data, &self.key, array_ref!(random_bytes, 0, 256))
    }
}

#[wasm_bindgen]
pub fn crc32(buffer: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(buffer);
    hasher.finalize()
}