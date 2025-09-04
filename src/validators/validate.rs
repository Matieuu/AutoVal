use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::{Field, Meta};

use crate::validators::max;

pub fn check_field(field: &Field) -> TokenStream {
    let name = &field.ident;
    let ty = &field.ty;

    let mut validation = quote! {};

    for attr in &field.attrs {
        if attr.path().is_ident("size") {
            if let Meta::List(list) = &attr.meta {
                let iter = &mut list.tokens.clone().into_iter();
                while let Some(token) = iter.next() {
                    match token {
                        TokenTree::Ident(ident) => {
                            if ident.to_string() == "max"
                                && let Some(TokenTree::Punct(_)) = iter.next()
                                && let Some(TokenTree::Literal(lit)) = iter.next()
                            {
                                let generated = max::check(&lit.into(), ty, name.as_ref().unwrap());
                                validation = quote! {
                                    #validation
                                    #generated
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }
            // let lit: LitInt = attr
            //     .parse_args()
            //     .expect("Expected integer literal in #[max(...)]");

            // let generated = vec![max::check(&lit, ty, name.as_ref().unwrap())];

            // validation = quote! {
            //     #validation

            //     #( #generated )*
            // }
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
