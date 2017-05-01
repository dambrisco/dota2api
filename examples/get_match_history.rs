extern crate dota2api;

use std::env::{ var, args };
use dota2api::Api;
use std::fmt;
use dota2api::response::MatchHistory;

fn main() {
    let key = match var("STEAM_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("You must have a STEAM_API_KEY environment configured to use this example"),
    };
    let api = Api::new(key);
    let vanity = args().skip(1).take(1).next().unwrap();
    match api.get_match_history(Vec::new()) {
        Ok(history) => println!("{}", history.result.matches.len()),
        Err(ref e) => println!("{}: {}", e.get_type(), e)
    }
}
