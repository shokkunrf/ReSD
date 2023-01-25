use wasm_bindgen::prelude::wasm_bindgen;

use crate::binary::Binary;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct FileHeaderSection {
    pub channel_count: u16,
    pub height: u32,
    pub width: u32,
    pub depth: u16,
    pub color_mode: u16,
}

pub fn get_file_header_section(binary: &mut Binary) -> FileHeaderSection {
    binary.set_position(12);

    FileHeaderSection {
        channel_count: binary
            .read_u16()
            .expect("Error in FileHeaderSection.channel_count"),
        height: binary
            .read_u32()
            .expect("Error in FileHeaderSection.height"),
        width: binary.read_u32().expect("Error in FileHeaderSection.width"),
        depth: binary.read_u16().expect("Error in FileHeaderSection.depth"),
        color_mode: binary
            .read_u16()
            .expect("Error in FileHeaderSection.color_mode"),
    }
}
