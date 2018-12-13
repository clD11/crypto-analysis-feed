extern crate hyper;
extern crate hyper_tls;

use std::io::{self, Write};
use std::fs;

use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::hyper::rt::{self, Future, Stream};

pub fn process_tweets() {
    // 1. create auth headers
    let auth_header = create_headers();

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

fn create_headers() -> String {

    //create sig, timestamp, tags

    let mut auth_header = String::from("OAuth ");
    // 1. add each value
    auth_header.push_str(r#"oauth_consumer_key="xvz1evFS4wEEPTGEFPHBog", "#);
    auth_header.push_str(r#"oauth_nonce="kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg", "#);
    auth_header.push_str(r#"oauth_signature="tnnArxj06cWHq44gCs1OSKk%2FjLY%3D", "#);
    auth_header.push_str(r#"oauth_signature_method="HMAC-SHA1", "#);
    auth_header.push_str(r#"oauth_timestamp="1318622958", "#);
    auth_header.push_str(r#"oauth_token="370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb", "#);
    auth_header.push_str(r#"oauth_version="1.0"#);
    
    String::from(auth_header)
}

#[cfg(test)]
mod tests {
    use super::create_headers;

    #[test]
    fn should_creat_valid_auth_headers() {
        let expected = r#"OAuth oauth_consumer_key="xvz1evFS4wEEPTGEFPHBog", oauth_nonce="kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg", oauth_signature="tnnArxj06cWHq44gCs1OSKk%2FjLY%3D", oauth_signature_method="HMAC-SHA1", oauth_timestamp="1318622958", oauth_token="370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb", oauth_version="1.0"#;
        let actual = create_headers();
        assert_eq!(actual, expected);
    }

}