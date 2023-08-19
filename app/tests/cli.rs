use resd::Resd;
use std::fs::File;
use std::io::Read;

#[test]
fn the_load() {
    let mut buf = Vec::new();

    let mut f = File::open("./tests/im_sasara.psd").expect("file not found");
    let _ = f.read_to_end(&mut buf).unwrap();

    let resd = Resd::new(buf);
    // let mut binary = binary::Binary::new(buf);
    // let resd = psd::Psd::get_psd(&mut binary);
    println!("{:?}", resd);
    // panic!("{}", resd.psd.file_header_section.channel_count);

    // assert_eq!(resd.file_header_section.channel_count, );
}

