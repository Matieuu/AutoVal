use proc_macro2::TokenTree;
use syn::{Attribute, Meta, Type};

pub fn is_string_type(ty: &Type) -> bool {
    match ty {
        Type::Path(tp) => {
            let ident = &tp.path.segments.last();
            if let Some(ident) = ident {
                ident.ident == "String"
            } else {
                false
            }
        }

        Type::Reference(r) => {
            if let Type::Path(tp) = &*r.elem
                && let Some(segment) = tp.path.segments.last()
            {
                segment.ident == "str"
            } else {
                false
            }
        }

        _ => false,
    }
}

pub fn is_numeric_type(ty: &Type) -> bool {
    if let Type::Path(typepath) = &ty {
        let ident = typepath.path.segments.last();
        if let Some(ident) = ident {
            matches!(
                ident.ident.to_string().as_str(),
                "u8" | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "f32"
                    | "f64"
            )
        } else {
            false
        }
    } else {
        false
    }
}

pub fn has_attribute(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(name))
}

pub fn has_token(attrs: &[Attribute], attr_name: &str, token_name: &str) -> bool {
    for attr in attrs {
        if attr.path().is_ident(attr_name)
            && let Meta::List(list) = &attr.meta
        {
            let iter = &mut list.tokens.clone().into_iter();
            for token in iter.by_ref() {
                if let TokenTree::Ident(ident) = token
                    && ident == token_name
                {
                    return true;
                }
            }
        }
    }

    false
}
