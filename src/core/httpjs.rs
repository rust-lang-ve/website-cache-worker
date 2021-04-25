use anyhow::{Context, Error, Result};
use async_trait::async_trait;
use http::{header::HeaderMap, StatusCode};
use http::{Method, Request, Response, Uri};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use url::{Position, Url};
use wasm_bindgen::JsValue;

#[async_trait]
pub trait Application {
    type RequestBody;
    type ResponseBody;

    fn new() -> Self;
    async fn handle(
        &self,
        request: Request<Self::RequestBody>,
    ) -> Result<Response<Self::ResponseBody>>;
}

#[derive(Debug, Deserialize)]
pub struct JsonHttpRequestValue<T> {
    headers: HashMap<String, String>,
    url: String,
    method: String,
    body: Option<T>,
}

impl<T> JsonHttpRequestValue<T>
where
    T: Clone + std::fmt::Debug + serde::de::DeserializeOwned,
{
    pub fn into_http_req(&self) -> Result<Request<Option<T>>> {
        let url = Url::from_str(self.url.as_str())?;
        let uri = Uri::from_str(&url[Position::BeforePath..])?;
        let method = Method::from_str(&self.method.to_uppercase())?;
        let headers = HeaderMap::try_from(&self.headers)?;

        let mut builder = Request::builder().uri(uri).method(method);

        let builder_headers = builder.headers_mut().unwrap();

        for (name, value) in headers {
            if let Some(header_name) = name {
                builder_headers.insert(header_name, value);
            }
        }

        if let Some(body) = self.body.clone() {
            return Ok(builder.body(Some(body))?);
        }

        Ok(builder.body(None)?)
    }

    pub fn from_jsvalue(value: JsValue) -> Result<Request<Option<T>>> {
        let json_request: JsonHttpRequestValue<T> = value.into_serde()?;
        let req: Request<Option<T>> = json_request.into_http_req()?;

        Ok(req)
    }
}

#[derive(Debug, Serialize)]
pub struct JsonHttpResponseValue<T> {
    headers: HashMap<String, String>,
    status_code: u16,
    body: T,
}

#[derive(Clone, Debug, Serialize)]
pub struct GenericErrorResponse {
    status_code: u16,
    message: String,
}

impl<T> JsonHttpResponseValue<T>
where
    T: Clone + std::fmt::Debug + serde::Serialize,
{
    pub fn json(status_code: StatusCode, body: T) -> Result<Self> {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(
            http::header::CONTENT_TYPE.to_string(),
            "application/json".to_string(),
        );

        Ok(JsonHttpResponseValue {
            status_code: status_code.as_u16(),
            headers,
            body,
        })
    }

    pub fn from_http_response(response: Response<T>) -> Result<Self> {
        let status_code = response.status().as_u16();
        let mut headers: HashMap<String, String> = HashMap::new();
        let body = response.body().clone();

        for (name, value) in response.headers() {
            headers.insert(name.to_string(), value.to_str().unwrap().to_string());
        }

        Ok(JsonHttpResponseValue {
            status_code,
            headers,
            body,
        })
    }

    pub fn to_jsvalue(&self) -> Result<JsValue> {
        JsValue::from_serde(&self).with_context(|| Error::msg("Failed to build http::Response"))
    }
}

pub fn generic_error_response(
    status: StatusCode,
    message: &str,
) -> Result<JsonHttpResponseValue<GenericErrorResponse>> {
    let body = GenericErrorResponse {
        status_code: status.as_u16(),
        message: message.into(),
    };

    JsonHttpResponseValue::json(status, body)
}
