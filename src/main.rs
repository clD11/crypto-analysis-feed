mod http;

use http::auth;

#[macro_use]
extern crate lazy_static;
extern crate yaml_rust;

use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug)]
struct OAuth {
   consumer_key: String,
   consumer_secret_key: String,
   nonce: String,
   signature_method: String,
   token: String,
   token_secret: String,
   version: String
}

pub struct TwitterConfig {
    stream_track_params: String,
    oauth: OAuth
}

lazy_static! {
    static ref TWITTER_CONFIG: TwitterConfig = {

        let file_content = fs::read_to_string("config.yaml")
            .expect("Cannot find config file");            
        let docs = YamlLoader::load_from_str(&file_content).unwrap();
        let twitter = &docs[0]["twitter"];
        let oauth = &twitter["oauth"];

        TwitterConfig {
            stream_track_params: String::from(twitter["stream_track_params"].as_str().unwrap()),
            oauth: OAuth {
                consumer_key: String::from(oauth["consumer_key"].as_str().unwrap()),
                consumer_secret_key: String::from(oauth["consumer_secret_key"].as_str().unwrap()),
                nonce: String::from(oauth["nonce"].as_str().unwrap()),
                signature_method: String::from(oauth["signature_method"].as_str().unwrap()),
                token: String::from(oauth["token"].as_str().unwrap()),
                token_secret: String::from(oauth["token_secret"].as_str().unwrap()),
                version: String::from("1.0")
            }            
        }
    };
}

fn main() {
    let signed_signature = auth::create_authorization_header(&TWITTER_CONFIG);
    println!("signature is {}", signed_signature);
}