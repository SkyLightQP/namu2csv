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
    let mut file = File::open("config/config.json").expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    contents
}

pub fn init() -> ConfigStructure {
    let config = &read_config();
    let json: ConfigStructure = serde_json::from_str(config)
        .expect("JSON was not well-formatted");

    json
}