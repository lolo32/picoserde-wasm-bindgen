use std::collections::HashMap;
use std::hash::Hash;

//use js_sys::{Array, ArrayBuffer, JsString, Number, Object, Uint8Array};
use js_sys::{Array, Object};
use wasm_bindgen::{JsCast, JsValue};

use super::{Result, static_str_to_js};

pub mod internal {
    use wasm_bindgen::prelude::*;

    use super::{Result, static_str_to_js};

    #[inline]
    pub fn label(label: &'static str) -> JsValue {
        static_str_to_js(label)
    }

    #[inline]
    pub fn obj_get(value: &JsValue, key: &'static str) -> Result<JsValue> {
        let key = label(key);
        Ok(js_sys::Reflect::get(value, &key).unwrap())
    }
}

pub trait DeJs: Sized {
    fn de_js(value: JsValue) -> Result<Self>;
}

fn is_nullish(value: &JsValue) -> bool {
    value.is_null() || value.is_undefined()
}

macro_rules! impl_ser_de_js_unsigned {
    ( $ ty: ident, $ max: expr) => {
        impl DeJs for $ty {
            #[inline]
            fn de_js(value: JsValue) -> Result<$ty> {
                match value.as_f64() {
                    Some(v) if (v as u64) > ($max as u64) => unimplemented!(),
                    Some(v) => Ok(v as $ty),
                    None => unimplemented!(),
                }
            }
        }
    };
}

macro_rules! impl_ser_de_js_signed {
    ( $ ty: ident, $ min: expr, $ max: expr) => {
        impl DeJs for $ty {
            #[inline]
            fn de_js(value: JsValue) -> Result<$ty> {
                match value.as_f64() {
                    Some(v) if (v as i64) < ($min as i64) || (v as i64) > ($max as i64) => unimplemented!(),
                    Some(v) => Ok(v as $ty),
                    None => unimplemented!(),
                }
            }
        }
    };
}

macro_rules! impl_ser_de_js_float {
    ( $ ty: ident) => {
        impl DeJs for $ty {
            #[inline]
            fn de_js(value: JsValue) -> Result<$ty> {
                match value.as_f64() {
                    Some(v) => Ok(v as $ty),
                    None => unimplemented!()
                }
            }
        }
    };
}

impl_ser_de_js_unsigned!(usize, std::usize::MAX);
impl_ser_de_js_unsigned!(u64, std::u64::MAX);
impl_ser_de_js_unsigned!(u32, std::u32::MAX);
impl_ser_de_js_unsigned!(u16, std::u16::MAX);
impl_ser_de_js_unsigned!(u8, std::u8::MAX);
impl_ser_de_js_signed!(i64, std::i64::MIN, std::i64::MAX);
impl_ser_de_js_signed!(i32, std::i32::MIN, std::i32::MAX);
impl_ser_de_js_signed!(i16, std::i16::MIN, std::i16::MAX);
impl_ser_de_js_signed!(i8, std::i8::MIN, std::i8::MAX);
impl_ser_de_js_float!(f64);
impl_ser_de_js_float!(f32);

impl<T> DeJs for Option<T>
    where
        T: DeJs,
{
    #[inline]
    fn de_js(value: JsValue) -> Result<Self> {
        if is_nullish(&value) {
            Ok(None)
        } else {
            Ok(Some(DeJs::de_js(value)?))
        }
    }
}

impl DeJs for bool {
    #[inline]
    fn de_js(value: JsValue) -> Result<Self> {
        match value.as_bool() {
            Some(v) => Ok(v),
            None => unimplemented!(),
        }
    }
}

impl DeJs for String {
    #[inline]
    fn de_js(value: JsValue) -> Result<Self> {
        match value.as_string() {
            Some(v) => Ok(v),
            None => unimplemented!(),
        }
    }
}

impl<T> DeJs for Vec<T>
    where
        T: DeJs,
{
    #[inline]
    fn de_js(value: JsValue) -> Result<Vec<T>> {
        let mut out = Vec::new();
        let array: &Array = value.dyn_ref::<Array>().unwrap();
        while array.length() > 0 {
            let item = array.shift();
            out.push(DeJs::de_js(item)?);
        }
        Ok(out)
    }
}

/*macro_rules!de_json_array_impl {
    ( $($count:expr),*) => {
        $(
        impl<T> DeJs for [T; $count] where T: DeJs {
            fn de_js(value: JsValue) -> Result<Self> {
                unsafe{
                    let mut to = std::mem::MaybeUninit::<[T; $count]>::uninit();
                    let top: *mut T = std::mem::transmute(&mut to);
                    de_json_array_impl_inner(top, $count, s, i)?;
                    Ok(to.assume_init())
                }
            }
        }
        )*
    }
}

de_json_array_impl!(
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32
);*/

impl<A, B> DeJs for (A, B)
    where
        A: DeJs,
        B: DeJs,
{
    fn de_js(value: JsValue) -> Result<Self> {
        if let Some(arr) = value.dyn_ref::<Array>() {
            Ok((
                DeJs::de_js(arr.get(0))?,
                DeJs::de_js(arr.get(1))?,
            ))
        } else {
            unimplemented!()
        }
    }
}

impl<A, B, C> DeJs for (A, B, C)
    where
        A: DeJs,
        B: DeJs,
        C: DeJs,
{
    fn de_js(value: JsValue) -> Result<Self> {
        if let Some(arr) = value.dyn_ref::<Array>() {
            Ok((
                DeJs::de_js(arr.get(0))?,
                DeJs::de_js(arr.get(1))?,
                DeJs::de_js(arr.get(2))?,
            ))
        } else {
            unimplemented!()
        }
    }
}

impl<A, B, C, D> DeJs for (A, B, C, D)
    where
        A: DeJs,
        B: DeJs,
        C: DeJs,
        D: DeJs,
{
    #[inline]
    fn de_js(value: JsValue) -> Result<Self> {
        if let Some(arr) = value.dyn_ref::<Array>() {
            Ok((
                DeJs::de_js(arr.get(0))?,
                DeJs::de_js(arr.get(1))?,
                DeJs::de_js(arr.get(2))?,
                DeJs::de_js(arr.get(3))?,
            ))
        } else {
            unimplemented!()
        }
    }
}

impl<K, V> DeJs for HashMap<K, V>
    where
        K: DeJs + Eq + Hash,
        V: DeJs,
{
    #[inline]
    fn de_js(value: JsValue) -> Result<Self> {
        if let Some(obj) = value.dyn_ref::<Object>() {
            let mut h = HashMap::new();
            for item in Object::entries(obj).iter() {
                let item = item.dyn_ref::<Array>().unwrap();
                let k = DeJs::de_js(item.get(0))?;
                let v = DeJs::de_js(item.get(1))?;
                h.insert(k, v);
            }
            Ok(h)
        } else {
            unimplemented!()
        }
    }
}

impl<T> DeJs for Box<T>
    where
        T: DeJs,
{
    #[inline]
    fn de_js(value: JsValue) -> Result<Box<T>> {
        Ok(Box::new(DeJs::de_js(value)?))
    }
}
