use wasm_bindgen::prelude::*;

/// A newtype that represents Serde errors as JavaScript exceptions.
#[derive(Debug)]
pub struct Error(JsValue);

#[derive(Debug)]
pub struct DeJsErr(JsValue);

impl std::fmt::Display for DeJsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = String)]
            pub fn to_string(value: &JsValue) -> String;
        }

        to_string(&self.0).fmt(f)
    }
}

impl std::error::Error for DeJsErr {}

impl DeJsErr {
    /// Creates a JavaScript `Error` with a given message.
    pub fn new<T: std::fmt::Display>(msg: T) -> Self {
        DeJsErr(js_sys::Error::new(&msg.to_string()).into())
    }
}

/// This conversion is needed for `?` to just work when using wasm-bindgen
/// imports that return JavaScript exceptions as `Result<T, JsValue>`.
impl From<JsValue> for DeJsErr {
    fn from(error: JsValue) -> DeJsErr {
        DeJsErr(error)
    }
}

// This conversion is needed for `?` to just work in wasm-bindgen exports
// that return `Result<T, JsValue>` to throw JavaScript exceptions.
impl From<DeJsErr> for JsValue {
    fn from(error: DeJsErr) -> JsValue {
        error.0
    }
}
