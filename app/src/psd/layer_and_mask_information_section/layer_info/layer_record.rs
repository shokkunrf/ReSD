mod channel_information;
mod coordinates;
mod layer_mask_data;

use self::{
    channel_information::{get_channel_information, ChannelInformation},
    coordinates::{get_coordinates, Coordinates},
    layer_mask_data::{get_layer_mask_data, LayerMaskData},
};
use crate::binary::Binary;

pub struct LayerRecord {
    pub coordinates: Coordinates,
    pub channel_count: u16,
    pub channel_informations: Vec<ChannelInformation>,
    pub blend_mode_key: String,
    pub opacity: u8,
    pub clipping: u8,
    pub extra_data_field_length: u32,
    pub layer_mask_data: LayerMaskData,
}

pub fn get_layer_record(binary: &mut Binary) -> LayerRecord {
    let coordinates = get_coordinates(binary);
    let channel_count = binary.read_u16().unwrap();

    let mut channel_informations = Vec::new();
    for _ in 0..channel_count {
        let channel_information = get_channel_information(binary);
        channel_informations.push(channel_information);
    }

    let sig = binary.read_utf8(4).unwrap();
    if sig != "8BIM" {
        panic!("LayerRecord.signature");
    }

    let blend_mode_key = binary.read_utf8(4).unwrap();
    match blend_mode_key.as_str() {
        "pass" | "norm" | "diss" | "dark" | "mul " | "idiv" | "lbrn" | "dkCl" | "lite" | "scrn"
        | "div " | "lddg" | "lgCl" | "over" | "sLit" | "hLit" | "vLit" | "lLit" | "pLit"
        | "hMix" | "diff" | "smud" | "fsub" | "fdiv" | "hue " | "sat " | "colr" | "lum " => (),
        _ => panic!(""),
    }

    let opacity = binary.read_u8().unwrap();

    let clipping = binary.read_u8().unwrap();
    match clipping {
        0 | 1 => (),
        _ => panic!(""),
    }

    let flags = binary.read_u8().unwrap();
    match flags {
        0 | 1 | 2 | 4 | 8 | 16 => (),
        _ => panic!(),
    }

    let filter = binary.read_u8().unwrap();
    match filter {
        0 => (),
        _ => panic!(),
    }

    let extra_data_field_length = binary.read_u32().unwrap();

    let layer_mask_data = get_layer_mask_data(binary);

    LayerRecord {
        coordinates,
        channel_count,
        channel_informations,
        blend_mode_key,
        opacity,
        clipping,
        extra_data_field_length,
        layer_mask_data,
    }
}
