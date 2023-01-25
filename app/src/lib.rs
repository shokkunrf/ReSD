mod binary;
mod psd;
mod utils;

use binary::Binary;
use psd::{get_psd, Psd};
use wasm_bindgen::prelude::*;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, resd!");
// }

// #[wasm_bindgen]
// pub fn get(bytes: Vec<u8>) -> Psd {
//     let mut binary = Binary::new(bytes);

//     get_psd(&mut binary)
// }
// ---

#[wasm_bindgen]
pub struct Resd {
    psd: Psd,
}

#[wasm_bindgen]
impl Resd {
    pub fn new(bytes: Vec<u8>) -> JsValue {
        let mut binary = Binary::new(bytes);

        let psd = get_psd(&mut binary);
        let resd = Resd { psd };
        JsValue::from(resd)
    }

    pub fn get_tree() {}
    pub fn rename_layers() {}
    // pub fn to_bytes() -> Vec<u8> {}
}
