use crate::binary::Binary;

pub struct ImageResourcesSection {
    pub length: u32,
}

pub fn get_image_resources_section(binary: &mut Binary) -> ImageResourcesSection {
    let length = binary
        .read_u32()
        .expect("Error in ImageResourcesSection.length");

    binary.increment_position(u64::from(length));
    ImageResourcesSection { length }
}
