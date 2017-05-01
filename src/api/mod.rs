pub mod error;
pub mod response;
mod url;
mod route;

use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use std::result::*;
use api::error::ApiError;
use api::url::Url;
use api::response::*;
use serde_json;

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
        let result = Api::get(
            Url::ResolveVanityUrl.value(),
            [("key", self.api_key.to_owned()), ("vanityurl", vanity_url)].to_vec());
        return match result {
            Err(e) => Err(e),
            Ok(s) => match serde_json::from_str::<Vanity>(&s)
                .map_err(|e| ApiError::JsonError(e)) {
                    Err(e) => Err(e),
                    Ok(r) => match r.response.message {
                        None => r.response.steam_id.parse::<u64>().map_err(|e| ApiError::ParseIntError(e)),
                        // Right now, we only expect Some(m) when there's an error
                        // This may change with API updates
                        Some(m) => Err(ApiError::StringError(m))
                    }
                }
        }
    }

    pub fn get_match_history(self, mut arguments: Vec<(&str, String)>) -> ApiResult<MatchHistory> {
        arguments.push(("key", self.api_key));
        let result = Api::get(
            Url::GetMatchHistory.value(),
            arguments);
        return match result {
            Err(e) => Err(e),
            Ok(s) => match serde_json::from_str::<MatchHistory>(&s)
                .map_err(|e| ApiError::JsonError(e)) {
                    Err(e) => Err(e),
                    Ok(s) => Ok(s)
                }
        }
    }

    fn get(mut url: hyper::Url, arguments: Vec<(&str, String)>) -> Result<String, ApiError> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        for item in arguments {
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
        return Ok(s);
    }
}
