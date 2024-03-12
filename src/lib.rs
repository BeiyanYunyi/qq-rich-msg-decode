mod utils;
use protobuf::Message;
use wasm_bindgen::prelude::*;
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}

#[wasm_bindgen(js_name = parsePicRec)]
pub fn parse_pic_rec(content: &[u8]) -> String {
    let result = RichMsg::PicRec::parse_from_bytes(content).expect("Failed to parse");
    return protobuf_json_mapping::print_to_string(&result).expect("Failed to convert to JSON");
}
