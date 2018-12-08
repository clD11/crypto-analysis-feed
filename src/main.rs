mod http;

use http::client;
use http::server;

fn main() {
    println!("making request");
    client::process_tweets();
    //server::init();
}