#[macro_use]
extern crate lazy_static;
extern crate yaml_rust;

use yaml_rust::YamlLoader;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct OAuth {
   consumer_key: String,
   nonce: String,
   signature: String,
   signature_method: String,
   timestamp: u64,
   token: String,
   version: String
}

struct TwitterConfig {
    oauth: OAuth
}

lazy_static! {
    static ref TWITTER_CONFIG: TwitterConfig = {

        let file_content = fs::read_to_string("config.yaml")
            .expect("Cannot find config file");            
        let docs = YamlLoader::load_from_str(&file_content).unwrap();
        let oauth = &docs[0]["twitter"]["oauth"];

        TwitterConfig {
            oauth: OAuth {
                consumer_key: String::from(oauth["consumer_key"].as_str().unwrap()),
                nonce: String::from(oauth["nonce"].as_str().unwrap()),
                signature: String::from("test"),
                signature_method: String::from(oauth["signature_method"].as_str().unwrap()),
                timestamp: Instant::now().elapsed().as_secs(),
                token: String::from(oauth["token"].as_str().unwrap()),
                version: String::from("1.0")
            }
        }
    };
}

fn main() {
    println!("twitter is {:#?}", TWITTER_CONFIG.oauth);
}