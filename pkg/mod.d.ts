/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} plaintext
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @returns {Uint8Array}
*/
export function ige_encrypt(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} ciphertext
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @returns {Uint8Array}
*/
export function ige_decrypt(ciphertext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} plaintext
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @returns {Uint8Array}
*/
export function ctr128(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} plaintext
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @returns {Uint8Array}
*/
export function ctr192(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} plaintext
* @param {Uint8Array} key
* @param {Uint8Array} iv
* @returns {Uint8Array}
*/
export function ctr256(plaintext: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;
