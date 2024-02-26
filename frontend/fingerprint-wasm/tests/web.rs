//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use fingerprint_wasm::encrypt_mod::{encrypt, decrypt};
use fingerprint_wasm::session_mod::make_fingerprint;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_encrypt_decrypt() {
    let data = "hello world";
    let secret = "123456";
    let encrypted = encrypt(data, secret);
    let decrypted = decrypt(&encrypted, secret);
    assert_eq!(data, decrypted);
}

#[wasm_bindgen_test]
fn test_make_fingerprint() {
    let fingerprint1 = make_fingerprint();
    let fingerprint2 = make_fingerprint();

    assert_eq!(fingerprint1, fingerprint2);
}