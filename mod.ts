import init, {
    source,
    // deno-lint-ignore camelcase
    ige_encrypt,
    // deno-lint-ignore camelcase
    ige_decrypt,
    ctr128 as _ctr128,
    ctr192 as _ctr192,
    ctr256 as _ctr256,
} from "./wasm.js";
  
  await init(source);

/**
 * Encrypt a plaintext array buffer using AES256-IGE.
 *
 * ```typescript
 * import { igeEncrypt } from "https://deno.land/x/wasm_crypto/mod.ts";
 * const text = new TextEncoder().encode("X".repeat(64));
 * const key  = new TextEncoder().encode("X".repeat(32));
 * const iv   = new TextEncoder().encode("X".repeat(32));
 * console.log(igeEncrypt(text, key, iv));
 * ```
 *
 * @param input Input data.
 */
export function igeEncrypt(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array {
    return ige_encrypt(plaintext, key, iv);
}

/**
 * Decrypt a ciphertext array buffer using AES256-IGE.
 *
 * ```typescript
 * import { igeDecrypt } from "https://deno.land/x/wasm_crypto/mod.ts";
 * const cipher = new TextEncoder().encode("X".repeat(64));
 * const key    = new TextEncoder().encode("X".repeat(32));
 * const iv     = new TextEncoder().encode("X".repeat(32));
 * console.log(igeDecrypt(cipher, key, iv));
 * ```
 *
 * @param input Input data.
 */
 export function igeDecrypt(ciphertext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array {
    return ige_decrypt(ciphertext, key, iv);
}

/**
 * Encrypt/decrypt an array buffer using AES-CTR128.
 *
 * ```typescript
 * import { ctr128 } from "https://deno.land/x/wasm_crypto/mod.ts";
 * const text = new TextEncoder().encode("X".repeat(64));
 * const key  = new TextEncoder().encode("X".repeat(16));
 * const iv   = new TextEncoder().encode("X".repeat(16));
 * console.log(ctr128(text, key, iv));
 * ```
 *
 * @param input Input data.
 */
 export function ctr128(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array {
    return _ctr128(plaintext, key, iv);
}

/**
 * Encrypt/decrypt an array buffer using AES-CTR192.
 *
 * ```typescript
 * import { ctr192 } from "https://deno.land/x/wasm_crypto/mod.ts";
 * const text = new TextEncoder().encode("X".repeat(64));
 * const key  = new TextEncoder().encode("X".repeat(24));
 * const iv   = new TextEncoder().encode("X".repeat(24));
 * console.log(ctr192(text, key, iv));
 * ```
 *
 * @param input Input data.
 */
 export function ctr192(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array {
    return _ctr192(plaintext, key, iv);
}

/**
 * Encrypt/decrypt an array buffer using AES-CTR256.
 *
 * ```typescript
 * import { ctr256 } from "https://deno.land/x/wasm_crypto/mod.ts";
 * const text = new TextEncoder().encode("X".repeat(64));
 * const key  = new TextEncoder().encode("X".repeat(32));
 * const iv   = new TextEncoder().encode("X".repeat(32));
 * console.log(ctr256(text, key, iv));
 * ```
 *
 * @param input Input data.
 */
 export function ctr256(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array {
    return _ctr256(plaintext, key, iv);
}