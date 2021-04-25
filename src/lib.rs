mod app;
mod core;

extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use http::StatusCode;
use wasm_bindgen::prelude::*;

use crate::core::{
    generic_error_response, Application, JsonHttpRequestValue, JsonHttpResponseValue,
};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[wasm_bindgen]
pub fn bootstrap() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub async fn handle_request(req: JsValue) -> Result<JsValue, JsValue> {
    // This is your application, find its implementation inside of the
    // src/app/mod.rs directory
    let app = app::App::new();

    // The JsValue representing the HTTP Request is deserialized into a
    // JsonHttpRequestValue which is able to create an instance of a
    // `http::Request` struct
    let request = JsonHttpRequestValue::<app::Payloads>::from_jsvalue(req).map_err(|e| {
        let message = format!("An error ocurred parsing the request. {}", e);

        // If we are unable to parse this request, its likely we doesn't support
        // it at all. Thus we return a `Bad Request (400)` by default
        generic_error_response(StatusCode::BAD_REQUEST, &message)
            .unwrap()
            .to_jsvalue()
            .unwrap()
    })?;

    // Here we handle the incoming request as an instance of `http::Request<T>`
    let response = app.handle(request).await.unwrap();

    // Here we turn the Response into a JSON friendly response
    let response = JsonHttpResponseValue::from_http_response(response).unwrap();

    // Turn such JsonHttpResponseValue into a JsValue so JavaScript is
    // able to handle it as expected
    let response = response.to_jsvalue().unwrap();

    // Here we bring the response to the Worker's JavaScript side
    Ok(response)
}
