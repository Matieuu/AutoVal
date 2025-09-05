use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::{Attribute, LitInt, LitStr, Type, parse_str};

/// Parsuje atrybut w stylu `#[xyz(mod = "crate")]`, `#[xyz(mod = "super")]`,
/// `#[xyz(mod = "self")]` albo `#[xyz(mod = "pub")]`.
/// Zwraca `Some(TokenStream)` z odpowiednim `pub(..)`. Brak -> None.
pub fn parse_visibility_from(
    attrs: &[Attribute],
    attr_name: &str,
) -> syn::Result<Option<TokenStream>> {
    let mut out: Option<TokenStream> = None;

    for attr in attrs {
        if !attr.path().is_ident(attr_name) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("mod") {
                let lit: LitStr = meta.value()?.parse()?;
                let s = lit.value();

                let ts = match s.as_str() {
                    "pub" => quote!(pub),
                    "crate" => quote!(pub(crate)),
                    "super" => quote!(pub(super)),
                    "self" => quote!(pub(self)),
                    "none" => quote! {},
                    // próbujemy ogólne `pub(#tokens)` jeśli ktoś poda np. "in crate::foo"
                    _ => {
                        let inner: TokenStream = s.parse().map_err(|_| {
                            syn::Error::new(lit.span(), "Non-standard value for `mod`")
                        })?;
                        quote!(pub(#inner))
                    }
                };
                out = Some(ts);
            }
            Ok(())
        })?;
    }

    Ok(out)
}

/// Zwraca domyślną widoczność, jeśli brak atrybutu: `pub`.
pub fn default_pub(vis_opt: Option<TokenStream>) -> TokenStream {
    vis_opt.unwrap_or_else(|| quote!(pub))
}

pub fn literal_for_type(ty: &str, lit: &LitInt) -> proc_macro2::TokenStream {
    match ty {
        // unsigned ints
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => {
            let value: u128 = lit.base10_parse().unwrap();
            let ty_parsed: Type = parse_str(ty).unwrap();
            quote! { #value as #ty_parsed }
        }

        // signed ints
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
            let value: i128 = lit.base10_parse().unwrap();
            let ty_parsed: Type = parse_str(ty).unwrap();
            quote! { #value as #ty_parsed }
        }

        // floats
        "f32" => {
            let value: f32 = lit.base10_parse().unwrap();
            quote! { #value as f32 }
        }
        "f64" => {
            let value: f64 = lit.base10_parse().unwrap();
            quote! { #value as f64 }
        }

        // fallback: zostaw jak jest (np. do warninga)
        _ => {
            let raw = lit.to_token_stream();
            quote! { #raw }
        }
    }
}

pub fn parse_field_attr_value(
    iter: &mut proc_macro2::token_stream::IntoIter,
    ident: &proc_macro2::Ident,
    name: &str,
) -> Result<proc_macro2::Literal, bool> {
    if ident == name {
        if let Some(TokenTree::Punct(_)) = iter.next() {
            if let Some(TokenTree::Literal(lit)) = iter.next() {
                Ok(lit)
            } else {
                Err(false)
            }
        } else {
            Err(false)
        }
    } else {
        Err(true)
    }
}
