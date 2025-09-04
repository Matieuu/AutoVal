use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitInt, Type};

use crate::{
    helpers::{is_compatible, is_numeric_type, is_string_type},
    utils::literal_for_type,
};

pub fn check(lit: &LitInt, ty: &Type, value: &Ident) -> TokenStream {
    if is_string_type(ty) {
        let max_val: usize = lit.base10_parse().expect("Invalid value in #[max] macro");
        quote! {
            if #value.len() > #max_val {
                return Err(format!("Field {} is too long (max {})", stringify!(#value), #max_val));
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
                let max_val = literal_for_type(&typename, lit);
                quote! {
                    if *#value > #max_val {
                        return Err(format!("Field {} is too big (max {})", stringify!(#value), #max_val));
                    }
                }
            }
        } else {
            unimplemented!("Support for Optionals, Vecs etc in near future")
        }
    } else {
        quote! {
            compile_error!(concat!("Attribute #[max] has no effect on field `", stringify!(#value), "`"));
        }
    }
}
