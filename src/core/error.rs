use serde::Serialize;
use std::string::ToString;
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize)]
pub enum Error {
    SerdeJson(String),
    EmptyBody(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn into_jsvalue(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e.to_string())
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::SerdeJson(msg) => msg.to_owned(),
            Error::EmptyBody(msg) => msg.to_owned(),
        }
    }
}
