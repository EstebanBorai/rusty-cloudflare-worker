extern crate cfg_if;
extern crate wasm_bindgen;

use crate::request::Request;
use crate::response::Response;

mod request;
mod response;
mod utils;

use cfg_if::cfg_if;
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

#[wasm_bindgen]
pub async fn http_handler(req: JsValue) -> Result<JsValue, JsValue> {
    let req = req.into_serde::<Request>().map_err(|e| e.to_string())?;

    let res = Response::from_str("Hello from Rust!");
    let res = JsValue::from_serde(&res).map_err(|e| e.to_string())?;

    Ok(res)
}
