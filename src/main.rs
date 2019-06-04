extern crate twitter_stream;
extern crate yaml_rust;
extern crate http;
extern crate regex;
extern crate serde_json;
extern crate serde;
extern crate base64;
extern crate reqwest;

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
use self::reqwest::*;

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

#[derive(Debug, Deserialize)]
struct BearerToken {
    token_type: String,
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct Trends {
    trends: Vec<Trend>,
    as_of: String,
    created_at: String,
    locations: Vec<Location>
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    woeid: i32
}

#[derive(Debug, Deserialize)]
struct Trend {
    name:               String,
//    url:                String,
//    promoted_content:   String,
//    query:              String,
//    tweet_volume:       i32
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
    let trending = find_trending(&config);

    println!("TRENDING NOW - {}", &trending);

    // TODO wrap in thread then listen to chanel kill after 60 min get trending and restart
    run_processor(trending, config);
}

fn run_processor(track_params: String, config: TwitterConfig) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("training_set.txt")
        .unwrap();

    let token = Token::new(config.oauth.consumer_key, config.oauth.consumer_secret,
                           config.oauth.token,config.oauth.token_secret);

    let future = TwitterStreamBuilder::filter(&token)
            .track(Some(&track_params[..]))
            .language(Some(&config.language[..]))
            .filter_level(Some(FilterLevel::None))
            .listen()
            .flatten_stream()
            .for_each(move|json| {

                let re = Regex::new(r".*promo|AIRDROP|airdrop|give away|free.*").unwrap();

                let tweet: Tweet = serde_json::from_str(&json).unwrap();

                if !re.is_match(&tweet.text[..]) {
                    let sanitized = tweet.text.replace("\n", "");
                    println!("{} - {}", tweet.created_at, sanitized);

                    if let Err(e) = writeln!(file, "{} - {}", tweet.created_at, &sanitized[..]) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
                Ok(())
            })
            .map_err(|e| println!("error: {}", e));

    rt::run(future);
}

fn find_trending(twitter_config: &TwitterConfig) -> String {

    // TODO Create as static so only retrieve once
    let client = reqwest::Client::new();
    let mut res = client.post("https://api.twitter.com/oauth2/token")
        .basic_auth(&twitter_config.oauth.consumer_key, Some(&twitter_config.oauth.consumer_secret))
        .header(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded;charset=UTF-8"))
        .body("grant_type=client_credentials")
        .send().unwrap();
    let content: BearerToken = res.json().unwrap();
    res.error_for_status().unwrap();

    // TODO Move into own fn
    res = client.get("https://api.twitter.com/1.1/trends/place.json?id=1")
        .bearer_auth(&content.access_token)
        .send().unwrap();

    let all_trends: Vec<Trends> = res.json().unwrap();
    // just get the first one for now
    all_trends[0].trends[0].name.clone()

//    for i in 0..all_trends.len() {
//        for j in 0..all_trends[i].trends.len() {
//            println!("{}", all_trends[i].trends[j].name);
//        }
//    }
}