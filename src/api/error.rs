use std::{ io, num, fmt, error };
use hyper;
use serde_json;

#[derive(Debug)]
pub enum ApiError {
    JsonError(serde_json::Error),
    IoError(io::Error),
    HyperError(hyper::Error),
    ParseIntError(num::ParseIntError),
    StringError(String),
    StrError(&'static str),
}

impl ApiError {
    pub fn get_type(&self) -> &str {
        use api::ApiError::*;

        match *self {
            JsonError(ref e) => "JsonError",
            IoError(ref e) => "IoError",
            HyperError(ref e) => "HyperError",
            ParseIntError(ref e) => "ParseIntError",
            StringError(ref s) => "StringError",
            StrError(ref s) => "StrError",
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use api::ApiError::*;

        match *self {
            JsonError(ref e) => e.fmt(f),
            IoError(ref e) => e.fmt(f),
            HyperError(ref e) => e.fmt(f),
            ParseIntError(ref e) => e.fmt(f),
            StringError(ref s) => write!(f, "{}", s),
            StrError(ref s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        use api::ApiError::*;

        match *self {
            JsonError(ref e) => e.description(),
            IoError(ref e) => e.description(),
            HyperError(ref e) => e.description(),
            ParseIntError(ref e) => e.description(),
            StringError(ref s) => s,
            StrError(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use api::ApiError::*;

        match *self {
            JsonError(ref e) => e.cause(),
            IoError(ref e) => e.cause(),
            HyperError(ref e) => e.cause(),
            ParseIntError(ref e) => e.cause(),
            StringError(_) => None,
            StrError(_) => None,
        }
    }
}
