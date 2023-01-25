mod layer_info;

use self::layer_info::{get_layer_info, LayerInfo};
use crate::binary::Binary;
use wasm_bindgen::prelude::wasm_bindgen;

// #[derive(Clone, Copy, Debug)]
// #[wasm_bindgen]
pub struct LayerAndMaskInformationSection {
    pub length: u32,
    pub layer_info: LayerInfo,
    // pub global_layer_mask_info: GlobalLayerMaskInfo,
    // pub additional_layer_informations: Vec<AdditionalLayerInformation>,
}

pub fn get_layer_and_mask_information_section(
    binary: &mut Binary,
) -> LayerAndMaskInformationSection {
    let length = binary
        .read_u32()
        .expect("Error in LayerAndMaskInformationSection.length");
    let layer_info = get_layer_info(binary);

    binary.set_position(u64::from(length));
    LayerAndMaskInformationSection {
        length,
        layer_info,
        // global_layer_mask_info: 0,
        // additional_layer_informations: 0,
    }
}
