#![no_std]

pub mod hashs;
mod utils;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(msg: &str) {
    alert(msg);
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}
