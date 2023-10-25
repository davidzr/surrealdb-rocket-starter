use std::{error, fmt};

use rocket::response::{status, Responder};
use rocket::serde::json::{json, Json};
use rocket::{response, Request,};

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    FileNotFound,
    OtherError(String),
    Surreal(surrealdb::Error),
}

pub type Result<T> = std::result::Result<T,Error>;

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidInput => write!(f, "Invalid input data."),
            Error::FileNotFound => write!(f, "File not found."),
            Error::Surreal(e) => write!(f, "{}", e),
            Error::OtherError(ref message) => write!(f, "An error occurred: {}", message),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(val: surrealdb::Error) -> Self {
        Error::Surreal(val)
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        let status = match self {
            Error::InvalidInput => rocket::http::Status::BadRequest,
            Error::FileNotFound => rocket::http::Status::NotFound,
            _ => rocket::http::Status::InternalServerError,
        };
        status::Custom(status, Json(json!({ "error": self.to_string() }))).respond_to(req)
    }
}