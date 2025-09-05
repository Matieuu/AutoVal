use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitInt, Type};

use crate::{
    helpers::{is_compatible, is_numeric_type, is_string_type},
    utils::literal_for_type,
};

pub fn check(lit: &LitInt, ty: &Type, value: &Ident) -> TokenStream {
    if is_string_type(ty) {
        let min_val: usize = lit
            .base10_parse()
            .expect("Invalid value in #[size(min = ...)] attribute");
        quote! {
            if #value.len() < #min_val {
                return Err(format!("Field {} is too short (min {})", stringify!(#value), #min_val));
            }
        }
    } else if is_numeric_type(ty) {
        if let Type::Path(typepath) = ty {
            let typename = typepath.path.segments.last().unwrap().ident.to_string();
            if is_compatible(&typename, lit).is_err() {
                quote! {
                    compile_error!(concat!("Field ", stringify!(#value), " is incompatible with type ", #typename));
                }
            } else {
                let min_val = literal_for_type(&typename, lit);
                quote! {
                    if *#value < #min_val {
                        return Err(format!("Field {} is too small (min {})", stringify!(#value), #min_val));
                    }
                }
            }
        } else {
            unimplemented!("Support for Optionals, Vecs etc in near future")
        }
    } else {
        quote! {
            compile_error!(concat!("Attribute #[size(min = ...)] has no effect on field `", stringify!(#value), "`"));
        }
    }
}
