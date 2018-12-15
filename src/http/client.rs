extern crate hyper;
extern crate hyper_tls;

use std::io::{self, Write};
use std::fs;

use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::hyper::rt::{self, Future, Stream};

pub fn process_tweets() {
    // 1. create auth headers
    // 2. setup filters
    // 3. make stream request

}

// temp refactor
fn stream() {
    rt::run(rt::lazy(|| {
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder()
            .build::<_, hyper::Body>(https);

        let uri = "https://hyper.rs".parse().unwrap();

        client
            // Fetch the url...
            .get(uri)
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
    }))
}

