# WASM Crypto

Various Rust crypto libraries wrapped in WASM for use in Deno and the browser.

# Usage

Be sure to build with `--unstable`.

```typescript
import { ige_encrypt, ige_decrypt } from 'https://deno.land/x/wasm_crypto/mod.js';

const key = Uint8Array.from([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f])

const iv = Uint8Array.from([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f])

const plaintext = Uint8Array.from([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f])

const ciphertext = ige_encrypt(plaintext, key, iv)
const decrypted  = ige_decrypt(ciphertext, key, iv)

console.log('Original value: ', plaintext)
console.log('Encrypted value:', ciphertext)
console.log('Decrypted value:', decrypted)
```

# Roadmap

The end goal for this project is to have all of the algorithms implemented that I possibly can. Since that will lead to very large WASM binaries though, I would like to eventually have different wasm/js files for different algorithm types (hashing, ciphers, etc). For now, however, everything will be bundled into a single binary.

- [ ] AES
  - [x] CBC (Counter)
  - [x] IGE (Infinite Garbled Extension)
  - [ ] CTR (Cipher Block Chaining)
  - [ ] ECB (Electronic Codebook)
- [ ] Bcrypt
- [ ] BLAKE2b
- [ ] BLAKE2s
- [ ] Blowfish
- [ ] ChaCha20
- [ ] Curve25519
- [ ] ECB, CBC, and CTR block cipher modes
- [ ] Ed25519
- [ ] Fortuna
- [ ] Ghash
- [ ] HC128
- [ ] HMAC
- [ ] MD5
- [ ] PBKDF2
- [ ] PKCS padding for CBC block cipher mode
- [ ] Poly1305
- [ ] RC4
- [ ] RIPEMD-160
- [ ] Salsa20 and XSalsa20
- [ ] Scrypt
- [ ] Sha1
- [ ] Sha2 (All fixed output size variants)
- [ ] Sha3
- [ ] Sosemanuk
- [ ] Whirlpool