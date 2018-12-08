mod core;

use core::client;
use core::server;

fn main() {
    println!("making request");
    client::init_client();
    //server::init();
}