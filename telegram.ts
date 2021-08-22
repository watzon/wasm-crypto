import {
    AuthKey as _AuthKey
} from './wasm.js';

export * from './mod.ts';

/**
 * Represents a 256 bit Telegram authorization key.
 * 
 * ```typescript
 * import { AuthKey } from "https://deno.land/x/wasm_crypto/mod.ts";
 * const bytes = new Uint8Array(256);
 * const key = AuthKey.from_bytes(bytes);
 * console.log(key.to_bytes);
 * ```
 */
 export class AuthKey {
    private _key: _AuthKey;

    private constructor(key: _AuthKey) {
        this._key = key;
    }

    /**
     * Create a new AuthKey from a byte array.
     * 
     * @param bytes
     * @returns {AuthKey}
     */
    static fromBytes(bytes: Uint8Array): AuthKey {
        const key = _AuthKey.from_bytes(bytes);
        return new AuthKey(key);
    }

    calcNewNonceHash(newNonce: Uint8Array, number: number) {
        return this._key.calc_new_nonce_hash(newNonce, number);
    }

    /**
     * Convert this key into its byte array representation.
     * 
     * @returns {Uint8Array}
     */
    toBytes(): Uint8Array {
        return this._key.to_bytes();
    }

    /**
     * Frees allocated memory. This class will not be usable after this is called.
     */
     free() {
        this._key.free();
    }
 }