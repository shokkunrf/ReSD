mod layer_blending_ranges_data;
mod layer_mask_data;

use self::{
    layer_blending_ranges_data::{get_layer_blending_ranges_data, LayerBlendingRangesData},
    layer_mask_data::{get_layer_mask_data, LayerMaskData},
};
use crate::binary::Binary;
use crate::psd::layer_and_mask_information_section::additional_layer_informations::{
    get_additional_layer_informations, AdditionalLayerInformations,
};

pub struct LayerRecord {
    pub coordinates: Coordinates,
    pub channel_count: u16,
    pub channel_informations: Vec<ChannelInformation>,
    pub blend_mode_key: String,
    pub opacity: u8,
    pub clipping: u8,
    pub extra_data_field_length: u32,
    pub layer_mask_data: LayerMaskData,
    pub layer_blending_ranges_data: LayerBlendingRangesData,
    pub layer_name: String,
    pub additional_layer_informations: AdditionalLayerInformations,
}

pub struct Coordinates {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub struct ChannelInformation {
    pub channel_id: i16,
    pub channel_data_length: u32,
}

pub fn get_layer_record(binary: &mut Binary) -> LayerRecord {
    let coordinates = Coordinates {
        top: binary.read_u32().unwrap(),
        left: binary.read_u32().unwrap(),
        bottom: binary.read_u32().unwrap(),
        right: binary.read_u32().unwrap(),
    };
    let channel_count = binary.read_u16().unwrap();

    let mut channel_informations = Vec::new();
    for _ in 0..channel_count {
        let channel_id = binary.read_i16().unwrap();
        match channel_id {
            -3..=9 => (),
            _ => panic!("ChannelInformation.channel_id"),
        };

        let channel_information = ChannelInformation {
            channel_id,
            channel_data_length: binary.read_u32().unwrap(),
        };

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

    let flag = binary.read_u8().unwrap();
    match flag {
        0 | 1 | 2 | 4 | 8 => (),
        _ => panic!("LayerRecord.flag"),
    }

    let filter = binary.read_u8().unwrap();
    match filter {
        0 => (),
        _ => panic!(),
    }

    let extra_data_field_length = binary.read_u32().unwrap();

    let layer_mask_data = get_layer_mask_data(binary);

    let layer_blending_ranges_data = get_layer_blending_ranges_data(binary);

    let layer_name_position = binary.get_position();
    let layer_name = binary.read_sjis_pascal_string().unwrap();

    let padding_mod = (binary.get_position() - layer_name_position) % 4;
    let padding = if padding_mod == 0 { 0 } else { 4 - padding_mod };
    binary.increment_position(padding);

    let additional_layer_informations = get_additional_layer_informations(binary);

    LayerRecord {
        coordinates,
        channel_count,
        channel_informations,
        blend_mode_key,
        opacity,
        clipping,
        extra_data_field_length,
        layer_mask_data,
        layer_blending_ranges_data,
        layer_name,
        additional_layer_informations,
    }
}
