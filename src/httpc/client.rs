extern crate hyper;
extern crate hyper_tls;

use std::io::{self, Write};
use std::fs;

use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::hyper::rt::{self, Future, Stream};
use httpc::auth::create_authorization_header;
use TwitterConfig;
use self::hyper::{Method, Request};
use http::{Request, Response};

pub fn process_tweets(twitter_config: &TwitterConfig) {
    rt::run(rt::lazy(|| {
        // 1. create auth headers
        let auth_header = create_authorization_header(&twitter_config);

        // 2. setup filters

        // 3. make stream request
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build::<_, _>(https);

        let uri = "https://stream.twitter.com/1.1/statuses/filter.json?track=bitcoin,ether".parse().unwrap();

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

