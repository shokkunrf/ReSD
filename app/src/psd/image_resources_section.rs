use wasm_bindgen::prelude::wasm_bindgen;

use crate::binary::Binary;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct ImageResourcesSection {
    pub length: u32,
}

pub fn get_image_resources_section(binary: &mut Binary) -> ImageResourcesSection {
    // let sec = ImageResourcesSection {
    //     length: binary
    //         .read_u32()
    //         .expect("Error in ImageResourcesSection.length"),
    // };

    // binary.increment_position(u64::from(sec.length));
    // sec

    let length = binary
        .read_u32()
        .expect("Error in ImageResourcesSection.length");

    binary.increment_position(u64::from(length));
    ImageResourcesSection { length }
}
