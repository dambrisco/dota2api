extern crate dota2api;

use std::env::*;
use dota2api::api::Api;

fn main() {
    let key = match var("STEAM_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("You must have a STEAM_API_KEY environment configured to use this example"),
    };
    let api = Api::new(key);
    let vanity = args().skip(1).take(1).next().unwrap();
    let steamid = api.resolve_vanity_url(vanity);
    println!("{}", steamid);
}