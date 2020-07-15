#![cfg_attr(feature = "external_doc", feature(external_doc))]
#![cfg_attr(feature = "external_doc", doc(include = "../README.md"))]
#![cfg_attr(feature = "external_doc", warn(missing_docs))]

use wasm_bindgen::prelude::*;

pub use de::DeJs;
pub use de::internal;
pub use error::DeJsErr;
pub use picoserde_derive_wasm_bindgen::{DeJs, SerJs};
pub use ser::SerJs;

mod de;
mod error;
mod ser;

type Result<T> = std::result::Result<T, DeJsErr>;

fn static_str_to_js(s: &'static str) -> JsValue {
    thread_local! {
        static CACHE: std::cell::RefCell<fnv::FnvHashMap<&'static str, JsValue>> = Default::default();
    }
    CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(s)
            .or_insert_with(|| JsValue::from_str(s))
            .clone()
    })
}

/// Converts [`JsValue`] into a Rust type.
pub fn from_value<T: DeJs>(value: JsValue) -> Result<T> {
    DeJs::de_js(value)
}

/// Converts a Rust value into a [`JsValue`].
pub fn to_value<T: SerJs>(value: &T) -> Result<JsValue> {
    Ok(SerJs::serialize_js(value))
}
