use anyhow::Result;
use async_trait::async_trait;
use http::Response;
use serde::Deserialize;

use crate::core::Application;

#[derive(Clone, Debug, Deserialize)]
pub struct Greet {
    from: String,
    body: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Payloads {
    JustText(String),
    Greeting(Greet),
}

pub struct App;

#[async_trait]
impl Application for App {
    type RequestBody = Option<Payloads>;
    type ResponseBody = String;

    fn new() -> Self {
        App
    }

    async fn handle(
        &self,
        request: http::Request<Self::RequestBody>,
    ) -> Result<Response<Self::ResponseBody>> {
        if let Some(body) = request.body() {
            return Ok(Response::builder()
                .status(http::StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(format!("Received a valid payload: {:?}", body))
                .expect("Unable to build body"));
        }

        Ok(Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(String::from(
                "Received no body from associated type \"Application::RequestBody\"",
            ))
            .expect("Unable to build body"))
    }
}
