use proc_macro::TokenStream;

use crate::parse::{Enum, Field, Struct};
use crate::shared;

pub fn derive_ser_js_proxy(proxy_type: &str, type_: &str) -> TokenStream {
    format!(
        "impl SerJs for {} {{
            fn ser_js(&self) -> JsValue {{
                let proxy: {} = self.into();
                proxy.ser_js()
            }}
        }}",
        type_,
        proxy_type
    )
        .parse()
        .unwrap()
}

pub fn derive_ser_js_struct(struct_: &Struct) -> TokenStream {
    let mut s = String::new();

    for field in &struct_.fields {
        let struct_fieldname = field.field_name.clone().unwrap();
        let js_fieldname =
            shared::attrs_rename(&field.attributes).unwrap_or_else(|| struct_fieldname.clone());

        if field.ty.is_option {
            l!(
                s,
                "if let Some(t) = &self.{} {{ object.set(SerJs::label(self, \"{}\"), t.ser_js()); }};",
                struct_fieldname,
                js_fieldname
            );
        } else {
            l!(
                s,
                "object.set(SerJs::label(self, \"{}\"), self.{}.ser_js());",
                js_fieldname,
                struct_fieldname
            );
        }
    }
    format!("const _: () = {{
    impl SerJs for {} {{
        fn ser_js(&self) -> JsValue {{
            let object = self.ser_object();
            {}
            object.into()
        }}
    }}
}};",
        struct_.name,
        s
    )
        .parse()
        .unwrap()
}

pub fn derive_de_js_named(name: &str, defaults: bool, fields: &[Field]) -> TokenStream {
    let mut local_vars = Vec::new();
    let mut struct_field_names = Vec::new();
    let mut js_field_names = Vec::new();
    let mut unwraps = Vec::new();

    let container_attr_default = defaults;

    let mut r = String::new();
    for field in fields {
        let struct_fieldname = field.field_name.as_ref().unwrap().to_string();
        let localvar = format!("_{}", struct_fieldname);
        let field_attr_default = shared::attrs_default(&field.attributes);
        let js_fieldname =
            shared::attrs_rename(&field.attributes).unwrap_or_else(|| struct_fieldname.clone());

        if field.ty.is_option {
            unwraps.push(format!(
                "{{ if let Some(t) = {} {{ t }} else {{ None }} }}",
                localvar
            ));
        } else if container_attr_default || field_attr_default {
            unwraps.push(format!(
                "{{ if let Some(t) = {} {{ t }} else {{ Default::default() }} }}",
                localvar
            ));
        } else {
            unwraps.push(format!(
                "{{ if let Some(t) = {} {{ t }} else {{unimplemented!(); /*return Err(s.err_nf(\"{}\"))*/}} }}",
                localvar,
                struct_fieldname
            ));
        }

        struct_field_names.push(struct_fieldname);
        js_field_names.push(js_fieldname);
        local_vars.push(localvar);
    }

    for local_var in &local_vars {
        l!(r, "let mut {} = None;", local_var);
    }

    for (js_field_name, local_var) in js_field_names.iter().zip(local_vars.iter()) {
        l!(
            r,
            "{} = Some(DeJs::de_js(picoserde_wasm_bindgen::internal::obj_get(&value, \"{}\")?)?);",
            local_var,
            js_field_name
        );
    }

    l!(r, "{} {{", name);
    for (field_name, unwrap) in struct_field_names.iter().zip(unwraps.iter()) {
        l!(r, "{}: {},", field_name, unwrap);
    }
    l!(r, "}");

    r.parse().unwrap()
}

pub fn derive_de_js_proxy(proxy_type: &str, type_: &str) -> TokenStream {
    format!(
        "const _: () = {{
            impl DeJs for {} {{
                fn de_js(value: JsValue) -> std::result::Result<Self, picoserde_wasm_bindgen::DeJsErr> {{
                    let proxy: {} = DeJs::de_js(value)?;
                    std::result::Result::Ok(Into::into(&proxy))
                }}
            }}
        }};",
        type_,
        proxy_type
    )
        .parse()
        .unwrap()
}

pub fn derive_de_js_struct(struct_: &Struct) -> TokenStream {
    let body = derive_de_js_named(
        &struct_.name,
        shared::attrs_default(&struct_.attributes),
        &struct_.fields[..],
    );

    format!(
        "const _: () = {{
            impl DeJs for {} {{
                fn de_js(value: JsValue) -> std::result::Result<Self, picoserde_wasm_bindgen::DeJsErr> {{
                    std::result::Result::Ok({{ {} }})
                }}
            }}
        }};",
        struct_.name,
        body
    )
        .parse()
        .unwrap()
}

// pub fn derive_ser_js_enum(input: &DeriveInput, enumeration: &DataEnum) -> TokenStream {
//     let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
//     let bound = parse_quote!(SerJs);
//     let bounded_where_clause = where_clause_with_bound(&input.generics, bound);

//     let ident = &input.ident;

//     let mut match_item = Vec::new();

//     for variant in &enumeration.variants {
//         let ident = &variant.ident;
//         let lit = LitStr::new(&ident.to_string(), ident.span());
//         match &variant.fields {
//             Fields::Unit => {
//                 match_item.push(quote!{
//                     Self::#ident => {s.label(#lit);s.out.push_str(":[]");},
//                 })
//             },
//             Fields::Named(fields_named) => {
//                 let mut items = Vec::new();
//                 let mut field_names = Vec::new();
//                 let last = fields_named.named.len() - 1;
//                 for (index, field) in fields_named.named.iter().enumerate() {
//                     if let Some(field_name) = &field.ident {
//                         let field_string = LitStr::new(&field_name.to_string(), field_name.span());
//                         if index == last{
//                             if type_is_option(&field.ty) {
//                                 items.push(quote!{if #field_name.is_some(){s.field(d+1, #field_string);#field_name.ser_js(d+1, s);}})
//                             }
//                             else{
//                                 items.push(quote!{s.field(d+1, #field_string);#field_name.ser_js(d+1, s);})
//                             }
//                         }
//                         else{
//                             if type_is_option(&field.ty) {
//                                 items.push(quote!{if #field_name.is_some(){s.field(d+1, #field_string);#field_name.ser_js(d+1, s);s.conl();}})
//                             }
//                             else{
//                                 items.push(quote!{s.field(d+1, #field_string);#field_name.ser_js(d+1, s);s.conl();})
//                             }
//                         }
//                         field_names.push(field_name);
//                     }
//                 }
//                 match_item.push(quote!{
//                     Self::#ident {#(#field_names,) *} => {
//                         s.label(#lit);
//                         s.out.push(':');
//                         s.st_pre();
//                         #(
//                             #items
//                         )*
//                         s.st_post(d);
//                     }
//                 });
//             },
//             Fields::Unnamed(fields_unnamed) => {
//                 let mut field_names = Vec::new();
//                 let mut str_names = Vec::new();
//                 let last = fields_unnamed.unnamed.len() - 1;
//                 for (index, field) in fields_unnamed.unnamed.iter().enumerate() {
//                     let field_name = Ident::new(&format!("f{}", index), field.span());
//                     if index != last{
//                         str_names.push(quote!{
//                             #field_name.ser_js(d, s); s.out.push(',');
//                         });
//                     }
//                     else{
//                         str_names.push(quote!{
//                             #field_name.ser_js(d, s);
//                         });
//                     }
//                     field_names.push(field_name);
//                 }
//                 match_item.push(quote!{
//                     Self::#ident (#(#field_names,) *) => {
//                         s.label(#lit);
//                         s.out.push(':');
//                         s.out.push('[');
//                         #(#str_names) *
//                         s.out.push(']');
//                     }
//                 });
//             },
//         }
//     }

//     quote! {
//         impl #impl_generics SerJs for #ident #ty_generics #bounded_where_clause {
//             fn ser_js(&self, d: usize, s: &mut makepad_tinyserde::SerJsState) {
//                 s.out.push('{');
//                 match self {
//                     #(
//                         #match_item
//                     ) *
//                 }
//                 s.out.push('}');
//             }
//         }
//     }
// }

pub fn derive_de_js_enum(enum_: &Enum) -> TokenStream {
    let mut r = String::new();

    for variant in &enum_.variants {
        // Unit
        if variant.fields.len() == 0 {
            l!(
                r,
                "\"{}\" => {{s.block_open(i)?;s.block_close(i)?;Self::{} }},",
                variant.name,
                variant.name
            );
        }
        // Named
        else if variant.named {
            let body =
                derive_de_js_named(&format!("Self::{}", variant.name), false, &variant.fields);
            l!(r, "\"{}\" => {{ {} }}, ", variant.name, body);
        }
        // Unnamed
        else if variant.named == false {
            let mut field_names = String::new();

            for _ in &variant.fields {
                l!(
                    field_names,
                    "{let r = DeJs::de_js(s,i)?;s.eat_comma_block(i)?;r},"
                );
            }
            l!(
                r,
                "\"{}\" => {{s.block_open(i)?;let r = Self::{}({}); s.block_close(i)?;r}}",
                variant.name,
                variant.name,
                field_names
            );
        }
    }

    format!(
        "const _: () = {{
            impl DeJs for {} {{
                fn de_js(svalue: JsValue) -> std::result::Result<Self, picoserde_wasm_bindgen::DeJsErr> {{
                    // we are expecting an identifier
                    s.curly_open(i)?;
                    let _ = s.string(i)?;
                    s.colon(i)?;
                    let r = std::result::Result::Ok(match s.strbuf.as_ref() {{
                {}
                        _ => return std::result::Result::Err(s.err_enum(&s.strbuf))
                    }});
                    s.curly_close(i)?;
                    r
                }}
            }}
        }};",
        enum_.name,
        r
    )
        .parse()
        .unwrap()
}

// pub fn derive_ser_js_struct_unnamed(input: &DeriveInput, fields:&FieldsUnnamed) -> TokenStream {
//     let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
//     let bound = parse_quote!(SerJs);
//     let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
//     let ident = &input.ident;

//     let mut str_names = Vec::new();
//     let last = fields.unnamed.len() - 1;
//     for (index, field) in fields.unnamed.iter().enumerate() {
//         let field_name = LitInt::new(&format!("{}", index), field.span());
//         if index != last{
//             str_names.push(quote!{
//                 self.#field_name.ser_js(d, s);
//                 s.out.push(',');
//             })
//         }
//         else{
//             str_names.push(quote!{
//                 self.#field_name.ser_js(d, s);
//             })
//         }
//     }
//     quote! {
//         impl #impl_generics SerJs for #ident #ty_generics #bounded_where_clause {
//             fn ser_js(&self, d: usize, s: &mut makepad_tinyserde::SerJsState) {
//                 s.out.push('[');
//                 #(
//                     #str_names
//                 ) *
//                 s.out.push(']');
//             }
//         }
//     }
// }

// pub fn derive_de_js_struct_unnamed(input: &DeriveInput, fields:&FieldsUnnamed) -> TokenStream {
//     let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
//     let ident = &input.ident;
//     let bound = parse_quote!(DeJs);
//     let bounded_where_clause = where_clause_with_bound(&input.generics, bound);

//     let mut items = Vec::new();
//     for _ in &fields.unnamed {
//         items.push(quote!{{let r = DeJs::de_js(s,i)?;s.eat_comma_block(i)?;r}});
//     }

//     quote! {
//         impl #impl_generics DeJs for #ident #ty_generics #bounded_where_clause {
//             fn de_js(s: &mut makepad_tinyserde::DeJsState, i: &mut std::str::Chars) -> std::result::Result<Self,DeJsErr> {
//                 s.block_open(i)?;
//                 let r = Self(
//                     #(
//                         #items
//                     ) *
//                 );
//                 s.block_close(i)?;
//                 std::result::Result::Ok(r)
//             }
//         }
//     }
// }