use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::{Field, Meta};

use crate::{
    utils::parse_field_attr_value,
    validators::{max, min},
};

pub fn check_field(field: &Field) -> TokenStream {
    let name = &field.ident;
    let ty = &field.ty;

    let mut validation = quote! {};

    for attr in &field.attrs {
        if attr.path().is_ident("size") {
            if let Meta::List(list) = &attr.meta {
                let iter = &mut list.tokens.clone().into_iter();
                while let Some(token) = iter.next() {
                    if let TokenTree::Ident(ident) = token {
                        if let Ok(lit) = parse_field_attr_value(iter, &ident, "max") {
                            let generated = max::check(&lit.into(), ty, name.as_ref().unwrap());
                            validation = quote! { #validation #generated };
                        }

                        if let Ok(lit) = parse_field_attr_value(iter, &ident, "min") {
                            let generated = min::check(&lit.into(), ty, name.as_ref().unwrap());
                            validation = quote! { #validation #generated };
                        }
                    }
                }
            }
        }
    }

    let func_name = format_ident!("validate_{}", name.as_ref().unwrap());
    validation = quote! {
        pub fn #func_name(#name: &#ty) -> Result<(), String> {
            #validation
            Ok(())
        }
    };

    validation
}
