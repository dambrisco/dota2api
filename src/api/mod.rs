pub mod error;
mod url;
mod route;

use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use json;
use json::JsonValue as Json;
use std::io::Read;
use std::result::*;
use api::error::ApiError;
use api::url::Url;

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
        let json =  Api::get(
            Url::ResolveVanityUrl.value(),
            &[("key", self.api_key.to_owned()), ("vanityurl", vanity_url)]);
        return match json {
            Err(e) => Err(e),
            Ok(json) => return match json["response"]["steamid"].as_str() {
                None => Err(ApiError::StrError("json structure did not match expected format")),
                Some(s) => match s.parse::<u64>() {
                    Err(e) => Err(ApiError::ParseIntError(e)),
                    Ok(u) => Ok(u)
                }
            }
        };
    }

    fn get(mut url: hyper::Url, arguments: &[(&str, String)]) -> Result<Json, ApiError> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        for item in arguments.iter() {
            url.query_pairs_mut().append_pair(item.0, &item.1);
        }
        let mut res = match client.get(url).send() {
            Ok(res) => res,
            Err(e) => return Err(ApiError::HyperError(e)),
        };
        let mut s = String::new();
        match res.read_to_string(&mut s) {
            Err(e) => return Err(ApiError::IoError(e)),
            Ok(_) => {}
        }
        return json::parse(&s).map_err(|e| ApiError::JsonError(e));
    }
}
