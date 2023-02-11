mod color_mode_data_section;
mod file_header_section;
mod image_resources_section;
mod layer_and_mask_information_section;

use self::{
    color_mode_data_section::{get_color_mode_data_section, ColorModeDataSection},
    file_header_section::{get_file_header_section, FileHeaderSection},
    image_resources_section::{get_image_resources_section, ImageResourcesSection},
    layer_and_mask_information_section::{
        get_layer_and_mask_information_section, LayerAndMaskInformationSection,
    },
};
use crate::binary::Binary;

pub struct Psd {
    pub file_header_section: FileHeaderSection,
    pub color_mode_data_section: ColorModeDataSection,
    pub image_resources_section: ImageResourcesSection,
    pub layer_and_mask_information_section: LayerAndMaskInformationSection,
}

pub fn get_psd(binary: &mut Binary) -> Psd {
    Psd {
        file_header_section: get_file_header_section(binary),
        color_mode_data_section: get_color_mode_data_section(binary),
        image_resources_section: get_image_resources_section(binary),
        layer_and_mask_information_section: get_layer_and_mask_information_section(binary),
    }
}
