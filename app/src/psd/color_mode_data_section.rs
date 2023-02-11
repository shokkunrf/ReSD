use crate::binary::Binary;

pub struct ColorModeDataSection {
    pub length: u32,
}

pub fn get_color_mode_data_section(binary: &mut Binary) -> ColorModeDataSection {
    let sec = ColorModeDataSection {
        length: binary
            .read_u32()
            .expect("Error in ColorModeDataSection.length"),
    };

    binary.increment_position(u64::from(sec.length));
    sec
}
