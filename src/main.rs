extern crate twitter_stream;
extern crate yaml_rust;
extern crate http;
extern crate regex;
extern crate serde_json;
extern crate serde;
extern crate base64;

mod top_client;

use self::yaml_rust::YamlLoader;
use self::http::Uri;
use std::fs;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use serde::de;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::prelude::*;
use twitter_stream::types::FilterLevel;
use regex::Regex;

use top_client::get_trending_topic;

#[derive(Deserialize)]
#[serde(untagged)]
enum StreamMessage {
    Tweet(Tweet),
    Other(de::IgnoredAny),
}

#[derive(Deserialize)]
struct Tweet {
    created_at: String,
    id: i64,
    text: String,
}

struct TwitterConfig {
    request_method: String,
    uri: Uri,
    track: String,
    location: String,
    language: String,
    oauth: OAuth
}

struct OAuth {
    consumer_key: String,
    consumer_secret: String,
    signature_method: String,
    token: String,
    token_secret: String,
    version: String
}

impl TwitterConfig {

    pub fn build(config_file: &String) -> TwitterConfig {
        let file_content = fs::read_to_string(&config_file).expect("Cannot find config file");
        let docs = YamlLoader::load_from_str(&file_content).unwrap();
        let twitter = &docs[0]["twitter"];
        let oauth = &twitter["oauth"];

        TwitterConfig {
            request_method: String::from(twitter["request_method"].as_str().unwrap()),
            uri: twitter["uri"].as_str().unwrap().parse::<Uri>().unwrap(),
            track: String::from(twitter["track"].as_str().unwrap()),
            location: String::from(twitter["location"].as_str().unwrap()),
            language: String::from(twitter["language"].as_str().unwrap()),
            oauth: OAuth {
                consumer_key: String::from(oauth["consumer_key"].as_str().unwrap()),
                consumer_secret: String::from(oauth["consumer_secret"].as_str().unwrap()),
                signature_method: String::from(oauth["signature_method"].as_str().unwrap()),
                token: String::from(oauth["token"].as_str().unwrap()),
                token_secret: String::from(oauth["token_secret"].as_str().unwrap()),
                version: String::from("1.0")
            }
        }
    }
}

fn main() {
    let config_resource = String::from("config.yaml");
    let config = TwitterConfig::build(&config_resource);

//    let token = Token::new(config.oauth.consumer_key, config.oauth.consumer_secret,
//                                    config.oauth.token,config.oauth.token_secret);

    top_client::get_trending_topic(&config);
    // obtain token
    // call trending endpoint
    // set track
    // start stream
    // after one hour start again

    // GET https://api.twitter.com/1.1/trends/place.json?id=1

//    let mut counter = 0;
//
//    let mut file = OpenOptions::new()
//        .write(true)
//        .append(true)
//        .open("training_set.txt")
//        .unwrap();
//
//    let future = TwitterStreamBuilder::filter(&token)
//        .track(Some(&config.track[..]))
//        .language(Some(&config.language[..]))
//        .filter_level(Some(FilterLevel::None))
//        .listen()
//        .flatten_stream()
//        .for_each(move|json| {
//
//            let re = Regex::new(r".*promo|AIRDROP|airdrop|give away|free.*").unwrap();
//
//            let tweet: Tweet = serde_json::from_str(&json).unwrap();
//
//            if !re.is_match(&tweet.text[..]) {
//                let sanitized = tweet.text.replace("\n", "");
//                println!("{} - {}", tweet.created_at, sanitized);
//
//                if counter < 100 {
//                    if let Err(e) = writeln!(file, "{} - {}", tweet.created_at, &sanitized[..]) {
//                        eprintln!("Couldn't write to file: {}", e);
//                    }
//                    counter += 1;
//                }
//                println!("{}", counter);
//            }
//            Ok(())
//        })
//        .map_err(|e| println!("error: {}", e));
//
//    rt::run(future);
}