use crate::binary::Binary;

pub struct LayerBlendingRangesData {
    pub length: u32,
    pub channel_sources: Vec<ChannelSource>,
}

pub struct ChannelSource {
    pub channel_source_range: u32, // b1,b2,w1,w2
    pub channel_destination_range: u32,
}

pub fn get_layer_blending_ranges_data(binary: &mut Binary) -> LayerBlendingRangesData {
    let length = binary.read_u32().unwrap();
    let count = length / 8;

    let mut channel_sources = Vec::new();
    for _ in 0..count {
        let channel_source = ChannelSource {
            channel_source_range: binary.read_u32().unwrap(),
            channel_destination_range: binary.read_u32().unwrap(),
        };
        channel_sources.push(channel_source);
    }

    LayerBlendingRangesData {
        length,
        channel_sources,
    }
}
