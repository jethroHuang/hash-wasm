extern crate alloc;
use core::fmt::Write;

use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sha256Hasher {
    hasher: Sha256,
}

#[wasm_bindgen]
impl Sha256Hasher {
    pub fn new() -> Sha256Hasher {
        let hasher = Sha256::new();
        Sha256Hasher { hasher }
    }

    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn digest(&mut self) -> alloc::string::String {
        let a = self.hasher.clone();
        let result = a.finalize();
        let mut text = alloc::string::String::new();
        write!(text, "{:x}", result).unwrap();
        text
    }
}

#[wasm_bindgen]
pub struct Md5Hasher {
    hasher: Md5,
}

#[wasm_bindgen]
impl Md5Hasher {
    pub fn new() -> Md5Hasher {
        let hasher = Md5::new();
        Md5Hasher { hasher }
    }

    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn digest(&mut self) -> alloc::string::String {
        let a = self.hasher.clone();
        let result = a.finalize();
        let mut text = alloc::string::String::new();
        write!(text, "{:x}", result).unwrap();
        text
    }
}

#[wasm_bindgen]
pub struct Sha1Hasher {
    hasher: Sha1,
}

#[wasm_bindgen]
impl Sha1Hasher {
    pub fn new() -> Sha1Hasher {
        let hasher = Sha1::new();
        Sha1Hasher { hasher }
    }

    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn digest(&mut self) -> alloc::string::String {
        let a = self.hasher.clone();
        let result = a.finalize();
        let mut text = alloc::string::String::new();
        write!(text, "{:x}", result).unwrap();
        text
    }
}

// sha512
#[wasm_bindgen]
pub struct Sha512Hasher {
    hasher: sha2::Sha512,
}

#[wasm_bindgen]
impl Sha512Hasher {
    pub fn new() -> Sha512Hasher {
        let hasher = sha2::Sha512::new();
        Sha512Hasher { hasher }
    }

    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    /// Returns the final hash result as a string of hexadecimal characters.
    pub fn digest(&mut self) -> alloc::string::String {
        let a = self.hasher.clone();
        let result = a.finalize();
        let mut text = alloc::string::String::new();
        write!(text, "{:x}", result).unwrap();
        text
    }
}
