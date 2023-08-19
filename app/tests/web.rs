//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::fs::File;
use std::io::Read;

use resd::Resd;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn resd_new() {
    let mut buf = Vec::new();

    let mut f = File::open("/app/tests/im_sasara.psd").expect("file not found");
    let _ = f.read_to_end(&mut buf).unwrap();

    let resd: JsValue = Resd::new(buf);
    println!("{:?}", resd);
    // panic!("{}", resd.psd.file_header_section.channel_count);

    // assert_eq!(resd.file_header_section.channel_count, );
}
