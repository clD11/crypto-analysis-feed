extern crate hyper;
extern crate hyper_tls;

use std::io::{self, Write};
use std::fs;

use self::hyper_tls::HttpsConnector;
use hyper::rt::{self, Future, Stream};
use hyper::{Client, Method, Request, Response};

pub fn get_trending_topic() {
    let https = hyper_tls::HttpsConnector::new(1).unwrap();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let uri: hyper::Uri = "https://api.twitter.com/oauth2/token";
    let mut req = Request::default();
    *req.method_mut() = Method::GET;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(hyper::header::AUTHORIZATION, auth_header.parse().unwrap());

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

