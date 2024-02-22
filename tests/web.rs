//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let a: u64 = 4194304000;
    let b: u64 = 524288000;
    let c: u64 = 4718592000;
    assert_eq!(a + b, c);
}