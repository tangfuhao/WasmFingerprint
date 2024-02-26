//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;


use std::assert;

use wasm_bindgen_test::{console_log, *};
use fingerprint_wasm::encrypt_mod::{encrypt, decrypt};
use fingerprint_wasm::session_mod::make_fingerprint;
use web_sys::js_sys::Math::random;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_encrypt_decrypt() {
    let data = "hello!!!!发多少JGAKS#&@*@HJworld";
    let secret = "sfsfs112212SHJSJSJSJS";
    let encrypted = encrypt(data, secret);
    let decrypted:Result<String, String> = decrypt(&encrypted, secret);
    assert_eq!(data, decrypted.unwrap());
}

#[wasm_bindgen_test]
fn test_encrypt_decrypt2() {
    let data = "helloJGAKS#&@*@HJworld";
    let secret = "12jdsdsjVNSKSKS3456";
    let encrypted = encrypt(data, secret);
    let edit_random_char_encrypted = encrypted.chars().enumerate().map(|(i, c)| {
        if i == 5 {
            'A'
        } else {
            c
        }
    }).collect::<String>();
    let decrypted = decrypt(&edit_random_char_encrypted, secret);
    assert!(decrypted.is_err());
}

#[wasm_bindgen_test]
fn test_make_fingerprint() {
    let fingerprint1 = make_fingerprint();
    let fingerprint2 = make_fingerprint();

    assert_eq!(fingerprint1, fingerprint2);
}