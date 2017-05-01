extern crate dota2api;

use std::env::{ var };
use dota2api::Api;

fn main() {
    let key = match var("STEAM_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("You must have a STEAM_API_KEY environment configured to use this example"),
    };
    let api = Api::new(key);
    match api.get_match_history(Vec::new()) {
        Ok(history) => println!("{}", history.result.matches.len()),
        Err(ref e) => println!("{}: {}", e.get_type(), e)
    }
}
