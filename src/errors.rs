//! Client errors

use hyper::Error as HttpError;
use hyper::StatusCode;
use hyper::error::UriError;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;

error_chain! {
    errors {
        Fault {
            code: StatusCode,
            error: ClientError,
        } {
            display("{}: '{}'", code, error.message)
            description(error.message.as_str())
          }
    }
    foreign_links {
        Codec(SerdeError);
        Http(HttpError);
        IO(IoError);
        URI(UriError);
    }
}

// representations

#[derive(Debug, Deserialize, PartialEq)]
pub struct FieldErr {
    pub resource: String,
    pub field: Option<String>,
    pub code: String,
    pub message: Option<String>,
    pub documentation_url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClientError {
    pub message: String,
    pub errors: Option<Vec<FieldErr>>,
}
