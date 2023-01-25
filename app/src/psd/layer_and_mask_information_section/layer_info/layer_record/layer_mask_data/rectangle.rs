use crate::binary::Binary;

pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub fn get_rectangle(binary: &mut Binary) -> Rectangle {
    Rectangle {
        top: binary.read_u32().unwrap(),
        left: binary.read_u32().unwrap(),
        bottom: binary.read_u32().unwrap(),
        right: binary.read_u32().unwrap(),
    }
}
