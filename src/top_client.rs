extern crate hyper;
extern crate hyper_tls;
extern crate base64;

use self::hyper::{Uri, Client, Method, Request, Response};
use self::hyper::rt::{self, Future, Stream};
use self::hyper_tls::HttpsConnector;
use self::base64::{encode, decode};

use std::io::{self, Write};
use std::fs;

use TwitterConfig;

pub fn get_trending_topic(twitter_config: &TwitterConfig) {
    let uri = "https://api.twitter.com/oauth2/token".parse::<Uri>().unwrap();;

    let secrets = format!("{}:{}", &twitter_config.oauth.consumer_key, &twitter_config.oauth.consumer_secret);
    let basic = format!("Basic {}", base64::encode(&secrets));

    let mut req = Request::default();
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(hyper::header::AUTHORIZATION, basic.parse().unwrap());

    let https = hyper_tls::HttpsConnector::new(1).unwrap();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    client.request(req).and_then(|res| {
        println!("Response: {}", res.status());
        res.into_body().for_each(|chunk| {
            io::stdout().write_all(&chunk)
                .map_err(|e| panic!("example expects stdout is open, error={}", e))
        })
    })
        .map_err(|err| {
            eprintln!("Error {}", err);
        });
}