extern crate proc_macro;

use crate::serde::*;

#[macro_use]
mod shared;

mod serde;
mod parse;

#[proc_macro_derive(SerJs, attributes(picoserde))]
pub fn derive_ser_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse::parse_data(input);

    if let Some(proxy) = shared::attrs_proxy(&input.attributes()) {
        return derive_ser_js_proxy(&proxy, &input.name());
    }

    // ok we have an ident, its either a struct or a enum
    match &input {
        parse::Data::Struct(struct_) if struct_.named => derive_ser_js_struct(struct_),
        //parse::Data::Enum(enum_) => derive_ser_js_enum(enum_),
        //parse::Data::Struct(struct_) => derive_ser_js_struct_unnamed(struct_),
        _ => unimplemented!("Only named structs are supported"),
    }
}

#[proc_macro_derive(DeJs, attributes(picoserde))]
pub fn derive_de_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse::parse_data(input);

    if let Some(proxy) = shared::attrs_proxy(&input.attributes()) {
        return derive_de_js_proxy(&proxy, &input.name());
    }

    // ok we have an ident, its either a struct or a enum
    match &input {
        parse::Data::Struct(struct_) => derive_de_js_struct(struct_),
        parse::Data::Enum(enum_) => derive_de_js_enum(enum_),
        parse::Data::Union(_) => unimplemented!("Unions are not supported"),
    }
}