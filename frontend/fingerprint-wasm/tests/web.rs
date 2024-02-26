//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use encrypt_mod::{encrypt, decrypt};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

//测试lib.rs下的加解密函数
#[wasm_bindgen_test]
fn test_encrypt_decrypt() {
    let data = "hello world";
    let secret = "123456";
    let encrypted = encrypt(data, secret);
    let decrypted = decrypt(&encrypted, secret);
    assert_eq!(data, decrypted);
}