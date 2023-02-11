use crate::binary::Binary;

pub struct LayerMaskData {
    pub length: u32,
    pub fields: Option<Fields>,
}

pub struct Fields {
    pub rectangle: Rectangle,
    pub default_color: u8, // 0 or 255
    pub flag: u8,
    pub mask: Option<Mask>,
    pub real: Option<Real>,
}

pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub struct Mask {
    pub flag: u8,
    pub parameter: MaskParameter,
}

pub enum MaskParameter {
    Density(i8),
    Feather(f64),
}

pub struct Real {
    pub flag: u8,
    pub user_mask_background: u8, // 0 or 255
    pub rectangle: Rectangle,
}

pub fn get_layer_mask_data(binary: &mut Binary) -> LayerMaskData {
    let length = binary.read_u32().unwrap();
    if length == 0 {
        return LayerMaskData {
            length,
            fields: None,
        };
    }

    let rectangle = Rectangle {
        top: binary.read_u32().unwrap(),
        left: binary.read_u32().unwrap(),
        bottom: binary.read_u32().unwrap(),
        right: binary.read_u32().unwrap(),
    };

    let default_color = binary.read_u8().unwrap();
    match default_color {
        0 | 255 => (),
        _ => panic!(""),
    }

    let flag = binary.read_u8().unwrap();
    match flag {
        0 | 1 | 2 | 4 | 8 => (),
        _ => panic!(""),
    }

    let mask: Option<Mask> = if flag == 8 {
        let mask_flag = binary.read_u8().unwrap();
        let mask_parameter = match mask_flag {
            0 | 2 => {
                let n = binary.read_i8().unwrap();
                MaskParameter::Density(n)
            }
            1 | 4 => {
                let n = binary.read_f64().unwrap();
                MaskParameter::Feather(n)
            }
            _ => panic!(""),
        };

        Some(Mask {
            flag: mask_flag,
            parameter: mask_parameter,
        })
    } else {
        None
    };

    if length == 20 {
        // padding
        // binary.increment_position(2);
        return LayerMaskData {
            length,
            fields: Some(Fields {
                rectangle,
                default_color,
                flag,
                mask,
                real: None,
            }),
        };
    }

    let real_flag = binary.read_u8().unwrap();
    match real_flag {
        0 | 1 | 2 | 4 | 8 => (),
        _ => panic!(""),
    }

    let real_user_mask_background = binary.read_u8().unwrap();
    match real_user_mask_background {
        0 | 255 => (),
        _ => panic!(""),
    }

    let real_rectangle = Rectangle {
        top: binary.read_u32().unwrap(),
        left: binary.read_u32().unwrap(),
        bottom: binary.read_u32().unwrap(),
        right: binary.read_u32().unwrap(),
    };

    LayerMaskData {
        length,
        fields: Some(Fields {
            rectangle,
            default_color,
            flag,
            mask,
            real: Some(Real {
                flag: real_flag,
                user_mask_background: real_user_mask_background,
                rectangle: real_rectangle,
            }),
        }),
    }
}
