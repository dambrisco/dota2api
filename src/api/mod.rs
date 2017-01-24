use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use std::result::*;
use json;
use json::JsonValue as Json;
use std::io;
use std::num;

static HOST: &'static str = "https://api.steampowered.com";
static VERSION: &'static str = "V001";

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub struct Api {
    api_key: String,
}

impl Api {
    pub fn new(api_key: String) -> Self {
        return Api {api_key: api_key};
    }

    pub fn resolve_vanity_url(self, vanity_url: String) -> ApiResult<u64> {
        let json =  Api::get(Url::ResolveVanityUrl.value(), &[("key".to_owned(), self.api_key.to_owned()), ("vanityurl".to_owned(), vanity_url)]);
        return match json {
            Err(e) => Err(e),
            Ok(json) => return match json["response"]["steamid"].as_str() {
                None => Err(ApiError::from_str("json structure did not match expected format")),
                Some(s) => match s.parse::<u64>() {
                    Err(e) => Err(ApiError::from_parseint(e)),
                    Ok(u) => Ok(u)
                }
            }
        };
    }

    fn get(mut url: hyper::Url, arguments: &[(String, String)]) -> Result<Json, ApiError> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        for item in arguments.iter() {
            url.query_pairs_mut().append_pair(&item.0, &item.1);
        }
        let mut res = match client.get(url).send() {
            Ok(res) => res,
            Err(e) => return Err(ApiError::from_hyper(e)),
        };
        let mut s = String::new();
        match res.read_to_string(&mut s) {
            Err(e) => return Err(ApiError::from_io(e)),
            Ok(_) => {}
        }
        return json::parse(&s).map_err(|e| ApiError::from_json(e));
    }
}

#[derive(Debug)]
pub enum Url {
    ResolveVanityUrl,
}

impl Url {
    fn value(&self) -> hyper::Url {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ApiError {
    JsonError(json::Error),
    IoError(io::Error),
    HyperError(hyper::Error),
    ParseIntError(num::ParseIntError),
    StringError(String)
}

impl ApiError {
    fn from_json(error: json::Error) -> ApiError {
        ApiError::JsonError(error)
    }

    fn from_io(error: io::Error) -> ApiError {
        ApiError::IoError(error)
    }

    fn from_hyper(error: hyper::Error) -> ApiError {
        ApiError::HyperError(error)
    }

    fn from_parseint(error: num::ParseIntError) -> ApiError {
        ApiError::ParseIntError(error)
    }

    fn from_str(error: &str) -> ApiError {
        ApiError::StringError(error.to_string())
    }
}
