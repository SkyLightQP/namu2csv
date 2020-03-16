use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ConfigStructure {
    pub urls: Vec<String>,
    pub ignore: IgnoreStructure,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct IgnoreStructure {
    pub words: Vec<String>,
    pub reverse: bool,
}

fn read_config() -> String {
    let mut file = File::open("config/config.json").expect("파일이 존재하지 않습니다.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("파일을 불러오는 중 오류가 발생하였습니다.");

    contents
}

pub fn init() -> ConfigStructure {
    let config = &read_config();
    let json: ConfigStructure = serde_json::from_str(config)
        .expect("JSON 문법이 올바르지 않습니다.");

    json
}