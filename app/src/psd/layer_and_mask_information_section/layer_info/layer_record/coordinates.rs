use crate::binary::Binary;
use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// #[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub fn get_coordinates(binary: &mut Binary) -> Coordinates {
    Coordinates {
        top: binary.read_u32().unwrap(),
        left: binary.read_u32().unwrap(),
        bottom: binary.read_u32().unwrap(),
        right: binary.read_u32().unwrap(),
    }
}
