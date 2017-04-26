use hyper;
use api::route::Route;

static HOST: &'static str = "https://api.steampowered.com";
static VERSION: &'static str = "V001";

#[derive(Debug)]
pub enum Url {
    ResolveVanityUrl,
}

impl Url {
    pub fn value(&self) -> hyper::Url {
        let path = match *self {
            Url::ResolveVanityUrl => "ResolveVanityURL",
        };
        let url_string = format!("{}/{}/{}/{}", HOST, Route::ISteamUser.value(), path, VERSION);
        let url = match hyper::Url::parse(&url_string) {
            Ok(url) => url,
            Err(_) => panic!("Unable to parse url"),
        };
        return url;
    }
}
