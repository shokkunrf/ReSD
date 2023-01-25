mod channel_image_data;
mod layer_record;

use self::{
    channel_image_data::ChannelImageData,
    layer_record::{get_layer_record, LayerRecord},
};
use crate::binary::Binary;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// #[wasm_bindgen]
pub struct LayerInfo {
    pub length: u32,
    pub layer_count: i16,
    // pub layer_records: JsValue,
    pub layer_records: Vec<LayerRecord>,
    // pub layer_records: Box<[JsValue]>,
    // pub channel_image_data: Vec<Vec<ChannelImageData>>,
}

pub fn get_layer_info(binary: &mut Binary) -> LayerInfo {
    let length = binary.read_u32().expect("Erorr in LayerInfo.length");
    let layer_count = binary.read_i16().expect("Erorr in LayerInfo.layer_count");

    let mut layer_records = Vec::new();
    for i in 0..layer_count.abs() {
        let layer_record = get_layer_record(binary);
        layer_records.push(layer_record);
    }

    LayerInfo {
        length,
        layer_count,
        layer_records: layer_records,
    }
}
