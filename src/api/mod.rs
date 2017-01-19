use hyper::Client;
use hyper::Url;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use rustc_serialize::json::Json;
use std::io::Read;
use std::result::Result::Ok;

static HOST: &'static str = "https://api.steampowered.com";
static VERSION: &'static str = "V001";

pub struct Api {
    api_key: String,
}

impl Api {
    pub fn new(api_key: String) -> Api {
        return Api {api_key: api_key};
    }

    pub fn resolve_vanity_url(self, vanity_url: String) -> u64 {
        return Api::get(TypedUrl::ResolveVanityUrl.value(), &[("key".to_owned(), self.api_key.to_owned()), ("vanityurl".to_owned(), vanity_url)]);
    }

    fn get(mut url: Url, arguments: &[(String, String)]) -> u64 {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        for item in arguments.iter() {
            url.query_pairs_mut().append_pair(&item.0, &item.1);
        }
        let url_string = url.to_string();
        let mut res = match client.get(url).send() {
            Ok(res) => res,
            Err(_) => panic!("request failed: {}", url_string),
        };
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        let data = Json::from_str(&s).unwrap();
        let string_id = match data.as_object().unwrap().get("response") {
            None => panic!("Unable to find field 'response'"),
            Some(obj) => match obj.as_object() {
                None => panic!("'response' is not an object"),
                Some(obj) => match obj.get("steamid") {
                    None => panic!("Unable to find field 'steamid'"),
                    Some(steamid) => match steamid.as_string() {
                        None => panic!("'steamid' is not a string"),
                        Some(steamid) => steamid,
                    },
                },
            },
        };
        let id: u64 = match string_id.parse() {
            Ok(u) => u,
            Err(_) => panic!("Unable to parse id"),
        };
        return id;
    }
}

enum TypedUrl {
    ResolveVanityUrl,
}

impl TypedUrl {
    fn value(&self) -> Url {
        let path = match *self {
            TypedUrl::ResolveVanityUrl => "ResolveVanityURL",
        };
        let url_string = format!("{}/{}/{}/{}", HOST, Route::ISteamUser.value(), path, VERSION);
        let url = match Url::parse(&url_string) {
            Ok(url) => url,
            Err(_) => panic!("Unable to parse url"),
        };
        return url;
    }
}

enum Route {
    ISteamUser,
}

impl Route {
    fn value(&self) -> String {
        return match *self {
            Route::ISteamUser => "ISteamUser".to_string(),
        };
    }
}
