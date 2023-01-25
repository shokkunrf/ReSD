mod rectangle;

use self::rectangle::{get_rectangle, Rectangle};
use crate::binary::Binary;

pub enum LayerMaskData {
    NotPresent(NotPresent),
    SomePresent(SomePresent),
    FullPresent(FullPresent),
}

pub struct NotPresent {
    pub layer_mask_data_length: u32,
}

pub struct Full {
    pub layer_mask_data_length: u32,
    pub rectangle: Rectangle,
    pub default_color: u8, // 0 or 255
                           // pub    flags: number | null = null;
                           // pub    maskParameters: number | null = null;
                           // pub  maskParametersFlags: number | null = null;
                           // pub padding: number | null = null;
                           // pub realFlags: number | null = null;
                           // pub realUserMaskBackground: number | null = null; // 0 or 255
                           // pub rectangleEnclosingLayerMask: Rectangle | null = null;
}

pub fn get_layer_mask_data(binary: &mut Binary) -> LayerMaskData {
    let layer_mask_data_length = binary.read_u32().unwrap();
    if layer_mask_data_length == 0 {
        return LayerMaskData::NotPresent(NotPresent {
            layer_mask_data_length,
        });
    }

    let rectangle = get_rectangle(binary);

    let default_color = binary.read_u8().unwrap();
    match default_color {
        0 | 255 => (),
        _ => panic!(""),
    }

    let flags = binary.read_u8().unwrap();

    LayerMaskData::Full(Full {
        layer_mask_data_length,
        rectangle,
        default_color,
    })
}
