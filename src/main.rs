mod http;

use http::client;
use http::server;

use std::fs;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    println!("making request");
    let file_content = fs::read_to_string("config.yaml").unwrap();

    let docs = YamlLoader::load_from_str(&file_content).unwrap();
    let doc = &docs[0]["twitter"];

    println!("Test - {}", doc["oauth"]["consumer_key"].as_str().unwrap());
}