mod httpjs;

extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use http::{Response, StatusCode};
use httpjs::{generic_error_response, JsonHttpRequestValue, JsonHttpResponseValue};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub async fn handle_request(req: JsValue) -> Result<JsValue, JsValue> {
    // The JsValue representing the HTTP Request is deserialized into a
    // JsonHttpRequestValue which is able to create an instance of a
    // `http::Request` struct
    let request = JsonHttpRequestValue::<Payloads>::from_jsvalue(req).map_err(|e| {
        let message = format!("An error ocurred parsing the request. {}", e);

        // If we are unable to parse this request, its likely we doesn't support
        // it at all. Thus we return a `Bad Request (400)` by default
        generic_error_response(StatusCode::BAD_REQUEST, &message)
            .unwrap()
            .to_jsvalue()
            .unwrap()
    })?;

    // Here we add some logic on our worker
    let name = if let Some(name) = request.body() {
        match name {
            Payloads::JustText(text) => text.to_owned(),
            Payloads::Greeting(greet) => greet.from.clone(),
        }
    } else {
        String::from("EmptyRusty")
    };

    // A `http::Response` is built to respond to this request
    let http_reponse = Response::builder()
        .status(200_u16)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(format!(
            "hello {}, from {}",
            name,
            request.uri().to_string()
        ))
        .expect("Failed to build http::Response");

    // Ceate an instance of JsonHttpResponseValue for the previously built
    // `http::Response` instance
    let response = JsonHttpResponseValue::from_http_response(http_reponse).unwrap();

    // Turn such JsonHttpResponseValue into a JsValue so JavaScript is
    // able to handle it as expected
    let response = response.to_jsvalue().unwrap();

    Ok(response)
}
