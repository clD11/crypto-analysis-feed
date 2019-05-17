mod core;

use core::{auth, twitclient};
use core::server;
use core::twitclient::process_tweets;

#[macro_use]
extern crate lazy_static;
extern crate yaml_rust;
extern crate twitter_stream;

use std::fs;
use yaml_rust::YamlLoader;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};

#[derive(Debug)]
struct OAuth {
    consumer_key: String,
    consumer_secret: String,
    nonce: String,
    signature_method: String,
    token: String,
    token_secret: String,
    version: String
}

pub struct TwitterConfig {
    request_method: String,
    stream_uri: String,
    stream_track_params: String,
    oauth: OAuth
}

//lazy_static! {
//    static ref TWITTER_CONFIG: TwitterConfig = {
//        let file_content = fs::read_to_string("config.yaml").expect("Cannot find config file");
//        let docs = YamlLoader::load_from_str(&file_content).unwrap();
//        let twitter = &docs[0]["twitter"];
//        let oauth = &twitter["oauth"];
//
//        TwitterConfig {
//            stream_track_params: String::from(twitter["stream_track_params"].as_str().unwrap()),
//            oauth: OAuth {
//                consumer_key: String::from(oauth["consumer_key"].as_str().unwrap()),
//                consumer_secret: String::from(oauth["consumer_secret"].as_str().unwrap()),
//                nonce: String::from(oauth["nonce"].as_str().unwrap()),
//                signature_method: String::from(oauth["signature_method"].as_str().unwrap()),
//                token: String::from(oauth["token"].as_str().unwrap()),
//                token_secret: String::from(oauth["token_secret"].as_str().unwrap()),
//                version: String::from("1.0")
//            }
//        }
//    };
//}

fn main() {
    let file_content = fs::read_to_string("config.yaml").expect("Cannot find config file");
    let docs = YamlLoader::load_from_str(&file_content).unwrap();
    let twitter = &docs[0]["twitter"];
    let oauth = &twitter["oauth"];

    let twitter_config = TwitterConfig {
        request_method: String::from(twitter["request_method"].as_str().unwrap()),
        stream_uri: String::from(twitter["stream_uri"].as_str().unwrap()),
        stream_track_params: String::from(twitter["stream_track_params"].as_str().unwrap()),
        oauth: OAuth {
            consumer_key: String::from(oauth["consumer_key"].as_str().unwrap()),
            consumer_secret: String::from(oauth["consumer_secret"].as_str().unwrap()),
            nonce: String::from(oauth["nonce"].as_str().unwrap()),
            signature_method: String::from(oauth["signature_method"].as_str().unwrap()),
            token: String::from(oauth["token"].as_str().unwrap()),
            token_secret: String::from(oauth["token_secret"].as_str().unwrap()),
            version: String::from("1.0")
        }
    };

    let token = Token::new(
        twitter_config.oauth.consumer_key,
        twitter_config.oauth.consumer_secret,
        twitter_config.oauth.token,
        twitter_config.oauth.token_secret);

    let future = TwitterStreamBuilder::filter(&token)
        .track(Some("bitcoin May 15"))
//        .locations("-122.75,36.8,-121.75,37.8")
        .listen()
        .flatten_stream()
        .for_each(|json| {
            println!("{}", json);
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    rt::run(future);
}