use std::collections::HashMap;

use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

use super::{ static_str_to_js};

//type Result<T = JsValue> = super::Result<T>;

mod internal {
    use wasm_bindgen::prelude::*;
    /// Custom bindings to avoid using fallible `Reflect` for plain objects.
    #[wasm_bindgen]
    extern "C" {
        pub type Object;

        #[wasm_bindgen(constructor)]
        pub fn new() -> Object;

        #[wasm_bindgen(method, indexing_setter)]
        pub fn set(this: &Object, key: JsValue, value: JsValue);
    }
}

pub type MyCustomJsObject = internal::Object;

pub trait SerJs {
    fn serialize_js(&self) -> JsValue {
        self.ser_js()
    }

    fn label(&self, label: &'static str) -> JsValue {
        static_str_to_js(label)
    }

    fn ser_object(&self) -> MyCustomJsObject {
        MyCustomJsObject::new()
    }

    fn ser_js(&self) -> JsValue;
}

macro_rules! impl_ser_de_json_unsigned {
    ( $ ty: ident, $ max: expr) => {
        impl SerJs for $ty {
            fn ser_js(&self) -> JsValue {
                assert!(self <= $max);
                JsValue::from_f64(*self as f64)
            }
        }
    };
}

macro_rules! impl_ser_de_json_signed {
    ( $ ty: ident, $ min: expr, $ max: expr) => {
        impl SerJs for $ty {
            fn ser_js(&self) -> JsValue {
                assert!(self >= $min);
                assert!(self <= $max);
                JsValue::from_f64(*self as f64)
            }
        }
    };
}

macro_rules! impl_ser_de_json_float {
    ( $ ty: ident, $min: expr, $ max: expr) => {
        impl SerJs for $ty {
            fn ser_js(&self) -> JsValue {
                assert!(self >= $min);
                assert!(self <= $max);
                JsValue::from_f64(*self as f64)
            }
        }
    };
}

impl_ser_de_json_unsigned!(usize, &std::usize::MAX);
impl_ser_de_json_unsigned!(u64, &std::u64::MAX);
impl_ser_de_json_unsigned!(u32, &std::u32::MAX);
impl_ser_de_json_unsigned!(u16, &std::u16::MAX);
impl_ser_de_json_unsigned!(u8, &std::u8::MAX);
impl_ser_de_json_signed!(i64, &std::i64::MIN, &std::i64::MAX);
impl_ser_de_json_signed!(i32, &std::i32::MIN, &std::i32::MAX);
impl_ser_de_json_signed!(i16, &std::i16::MIN, &std::i16::MAX);
impl_ser_de_json_signed!(i8, &std::i8::MIN, &std::i8::MAX);
impl_ser_de_json_float!(f64, &std::f64::MIN, &std::f64::MAX);
impl_ser_de_json_float!(f32, &std::f32::MIN, &std::f32::MAX);

impl<T> SerJs for Option<T>
    where
        T: SerJs,
{
    fn ser_js(&self) -> JsValue {
        if let Some(v) = self {
            v.ser_js()
        } else {
            JsValue::UNDEFINED
        }
    }
}

impl SerJs for bool {
    fn ser_js(&self) -> JsValue {
        match *self {
            true => JsValue::TRUE,
            false => JsValue::FALSE
        }
    }
}

impl SerJs for String {
    fn ser_js(&self) -> JsValue {
        JsString::from((*self).clone()).into()
    }
}

impl<T> SerJs for Vec<T>
    where
        T: SerJs,
{
    fn ser_js(&self) -> JsValue {
        let array = Array::new();
        for item in self {
            array.push(&item.ser_js());
        }
        array.into()
    }
}

impl<T> SerJs for [T]
    where
        T: SerJs,
{
    fn ser_js(&self) -> JsValue {
        let array = Array::new();
        for item in self {
            array.push(&item.ser_js());
        }
        array.into()
    }
}

impl<A, B> SerJs for (A, B)
    where
        A: SerJs,
        B: SerJs,
{
    fn ser_js(&self) -> JsValue {
        let array = Array::new();
        array.push(&self.0.ser_js());
        array.push(&self.1.ser_js());
        array.into()
    }
}

impl<A, B, C> SerJs for (A, B, C)
    where
        A: SerJs,
        B: SerJs,
        C: SerJs,
{
    fn ser_js(&self) -> JsValue {
        let array = Array::new();
        array.push(&self.0.ser_js());
        array.push(&self.1.ser_js());
        array.push(&self.2.ser_js());
        array.into()
    }
}

impl<A, B, C, D> SerJs for (A, B, C, D)
    where
        A: SerJs,
        B: SerJs,
        C: SerJs,
        D: SerJs,
{
    fn ser_js(&self) -> JsValue {
        let array = Array::new();
        array.push(&self.0.ser_js());
        array.push(&self.1.ser_js());
        array.push(&self.2.ser_js());
        array.push(&self.3.ser_js());
        array.into()
    }
}

impl<K, V> SerJs for HashMap<K, V>
    where
        K: SerJs,
        V: SerJs,
{
    fn ser_js(&self) -> JsValue {
        let object = self.ser_object();
        for (k, v) in self {
            object.set(k.ser_js(), v.ser_js());
        }
        object.into()
    }
}

impl<T> SerJs for Box<T>
    where
        T: SerJs,
{
    fn ser_js(&self) -> JsValue {
        (**self).ser_js()
    }
}
