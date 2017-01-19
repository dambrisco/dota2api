extern crate dota2api_rust;

use std::env::*;

fn main() {
    let key = match var("STEAM_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("You must have a STEAM_API_KEY environment configured to use this example"),
    };
    let vanity = args().skip(1).take(1).next().unwrap();
    let steamid = dota2api_rust::resolve_vanity_url(key, vanity);
    println!("{}", steamid);
}
