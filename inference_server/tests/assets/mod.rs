use base64::prelude::*;
use std::fs::File;
use std::io::Read;

pub fn pig_base64() -> String {
    read_file_as_base64("./tests/assets/pig.jpg")
}

pub fn read_file_as_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let base64_payload = BASE64_STANDARD.encode(contents);

    format!("data:image/jpeg;base64,{}", base64_payload)
}
