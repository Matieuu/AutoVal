use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, GenericArgument, LitStr, PathArguments, Result, Type, TypePath};

/// Parsuje atrybut w stylu `#[xyz(mod = "crate")]`, `#[xyz(mod = "super")]`,
/// `#[xyz(mod = "self")]` albo `#[xyz(mod = "pub")]`.
/// Zwraca `Some(TokenStream)` z odpowiednim `pub(..)`. Brak -> None.
pub fn parse_visibility_from(attrs: &[Attribute], attr_name: &str) -> Result<Option<TokenStream>> {
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
                    // próbujemy ogólne `pub(#tokens)` jeśli ktoś poda np. "in crate::foo"
                    _ => {
                        let inner: TokenStream = s.parse().map_err(|_| {
                            syn::Error::new(lit.span(), "Niepoprawna wartość dla `mod`")
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

pub fn is_option(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(_) = &args.args[0] {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}
