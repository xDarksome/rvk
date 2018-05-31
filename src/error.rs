//! All errors that can happen during a method call

use std::fmt::{self, Display, Formatter};
use std::result::Result as StdResult;

/// Convenience type for defining `Result`s
pub type Result<T> = StdResult<T, Error>;

/// An error returned by the API
#[derive(Deserialize, Debug)]
pub struct APIError {
    code: u64,
    msg: String,
}

impl APIError {
    /// Creates a new `APIError`
    pub fn new(code: u64, msg: String) -> APIError {
        APIError { code, msg }
    }

    /// Returns the code of this `APIError`
    pub fn code(&self) -> u64 {
        self.code
    }

    /// Returns the message of this `APIError`
    pub fn msg(&self) -> &String {
        &self.msg
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), fmt::Error> {
        f.write_str(&format!("APIError #{}: {}", self.code(), self.msg()))
    }
}

/// A generic error
#[derive(Debug)]
pub enum Error {
    /// Errors from the API
    API(APIError),

    /// Errors with making a request
    Request(::reqwest::Error),

    /// Serde Errors
    Serde(::serde_json::error::Error),

    /// Other errors
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), fmt::Error> {
        match self {
            Error::API(e) => e.fmt(f),
            Error::Request(e) => e.fmt(f),
            Error::Serde(e) => e.fmt(f),
            Error::Other(s) => f.write_str(&s),
        }
    }
}

impl From<APIError> for Error {
    fn from(e: APIError) -> Error {
        Error::API(e)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(e: ::reqwest::Error) -> Error {
        Error::Request(e)
    }
}

impl From<::serde_json::error::Error> for Error {
    fn from(e: ::serde_json::error::Error) -> Error {
        Error::Serde(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Other(s)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Error {
        s.to_string().into()
    }
}
