use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::utils::{
    checker::has_token,
    parser::{parse_named_fields, parse_optional_field},
};

pub fn generate(input: &DeriveInput) -> TokenStream {
    if ["setters", "getters"]
        .iter()
        .all(|&token| !has_token(&input.attrs, "autoval", token))
    {
        return TokenStream::new();
    }

    let input_ident = &input.ident;
    let fields = parse_named_fields(input);

    let mut setter_funcs: Vec<TokenStream> = Vec::new();
    let mut getter_funcs: Vec<TokenStream> = Vec::new();

    for field in fields {
        let field_ident = &field.ident;
        let field_type = &field.ty;
        let (parsed_type, is_optional) = parse_optional_field(field);

        let owned_getter_ident = if let Some(field_ident) = field_ident.as_ref() {
            format_ident!("{}_owned", field_ident)
        } else {
            abort!(
                field_ident,
                "Couldn't create owned getter function for field"
            )
        };

        getter_funcs.push(quote! {
            pub fn #field_ident(&self) -> &#field_type {
                &self.#field_ident
            }

            pub fn #owned_getter_ident(&self) -> <#field_type as ::std::borrow::ToOwned>::Owned
            where #field_type: ::std::borrow::ToOwned {
                self.#field_ident.to_owned()
            }
        });
    }

    quote! {
        impl #input_ident {
            #( #getter_funcs )*
        }
    }
}
