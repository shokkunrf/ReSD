mod binary;
mod psd;
mod utils;

use binary::Binary;
use psd::{get_psd, Psd};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, resd!");
// }

#[wasm_bindgen]
pub struct Resd {
    #[allow(dead_code)]
    psd: Psd,
}

#[wasm_bindgen]
impl Resd {
    pub fn new(bytes: Vec<u8>) -> JsValue {
        set_panic_hook();
        let mut binary = Binary::new(bytes);
        let psd = get_psd(&mut binary);
        let resd = Resd { psd };
        JsValue::from(resd)

        // let psd1 = get_psd(&mut binary);
        // let psd = match psd1 {
        //     Ok(psd) => psd,
        //     Err(e) => {
        //         alert(e.to_string().as_str());
        //         panic!("");
        //     }
        // };
    }

    pub fn get_tree() {}
    pub fn rename_layers() {}
    // 必要な量だけ取得したPsdと出力に必要な元のバイナリを合わせてbytesにする。
    // pub fn to_bytes() -> Vec<u8> {}
}

#[cfg(test)]
mod tests {
    use super::Resd;
    use std::{fs::File, io::Read};

    #[test]
    fn new() {
        let mut buf = Vec::new();
        let mut f = File::open("/app/tests/img.psd").expect("file not found");
        let _ = f.read_to_end(&mut buf).unwrap();

        let resd = Resd::new(buf);
        assert_eq!(resd, 0);
    }
}
