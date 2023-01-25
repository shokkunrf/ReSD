use crate::binary::Binary;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// #[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ChannelInformation {
    pub channel_id: i16,
    pub channel_data_length: u32,
}

pub fn get_channel_information(binary: &mut Binary) -> ChannelInformation {
    let channel_id = binary.read_i16().unwrap();
    match channel_id {
        -3..=9 => (),
        _ => panic!("ChannelInformation.channel_id"),
    };

    ChannelInformation {
        channel_id,
        channel_data_length: binary.read_u32().unwrap(),
    }
}
