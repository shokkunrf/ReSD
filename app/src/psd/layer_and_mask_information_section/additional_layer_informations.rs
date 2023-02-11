use crate::binary::Binary;

pub type AdditionalLayerInformations = Vec<AdditionalLayerInformation>;

pub struct AdditionalLayerInformation {
    pub key: String,
    pub data_length: u32,
    pub data: Data,
}

pub enum Data {
    Luni(Luni),
    NotImplemented(Option<()>),
}

pub fn get_additional_layer_informations(binary: &mut Binary) -> AdditionalLayerInformations {
    let mut additional_layer_informations = Vec::new();

    loop {
        let signature = binary.read_utf8(4).unwrap();
        if signature != "8BIM" {
            return additional_layer_informations;
        }

        let key = binary.read_utf8(4).unwrap();

        let data_length = binary.read_u32().unwrap();

        let data = match key.as_str() {
            "luni" => Data::Luni(get_luni(binary)),
            _ => {
                binary.increment_position(data_length.into());
                Data::NotImplemented(None)
            }
        };

        let additional_layer_information = AdditionalLayerInformation {
            key,
            data_length,
            data,
        };

        additional_layer_informations.push(additional_layer_information);
    }
}

pub struct Luni {
    pub length: u32,
    pub text: String,
}

fn get_luni(binary: &mut Binary) -> Luni {
    let pos = binary.get_position();
    let text = binary.read_unicode_string().unwrap();
    let length: u32 = (binary.get_position() - pos) as u32;
    // let length: u32 = (binary.get_position() - pos).try_into().unwrap();

    Luni { length, text }
}
