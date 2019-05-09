extern crate hyper;
extern crate hyper_tls;
extern crate http;

use std::io::{self, Write};
use std::fs;
//use http::header::HeaderMap;

//use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::hyper::rt::{self, Future, Stream};
use self::hyper::{Client, Method, Request, Response};

use core::auth::create_authorization_header;
use TwitterConfig;

pub fn process_tweets(twitter_config: &TwitterConfig) {
    // 1. create auth headers
    let auth_header = create_authorization_header(&twitter_config);

    println!("{}", auth_header);

    rt::run(rt::lazy(move || {
        // 2. setup filters

        // 3. make stream request
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        //let mut map = HeaderMap::new();

        let uri: hyper::Uri = "https://stream.twitter.com/1.1/statuses/filter.json?track=bitcoin,ether".parse().unwrap();
        let mut req = Request::default();
        *req.method_mut() = Method::GET;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::AUTHORIZATION,
            auth_header.parse().unwrap()
        );

        client.request(req)
            // And then, if we get a response back...
            .and_then(|res| {
                println!("Response: {}", res.status());
                println!("Headers: {:#?}", res.headers());

                // The body is a stream, and for_each returns a new Future
                // when the stream is finished, and calls the closure on
                // each chunk of the body...
                res.into_body().for_each(|chunk| {
                    io::stdout().write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })
            })
            // If all good, just tell the user...
            .map(|_| {
                println!("\n\nDone.");
            })
            // If there was an error, let the user know...
            .map_err(|err| {
                eprintln!("Error {}", err);
            })
    }));
}

