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
 export class AuthKey extends _AuthKey {}