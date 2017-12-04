//! Client errors

use hyper::Error as HttpError;
use hyper::StatusCode;
use hyper::error::UriError;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;

use Response;
use std::collections::HashMap;

error_chain! {
    errors {
        Fault {
            code: StatusCode,
            error: Response<HashMap<String, String>>,
        } {
            display("{}: '{}'", code, error.clone().meta.error_detail.unwrap())
            description("unknown error")
          }
    }
    foreign_links {
        Codec(SerdeError);
        Http(HttpError);
        IO(IoError);
        URI(UriError);
    }
}