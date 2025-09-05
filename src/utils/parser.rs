use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, punctuated::Punctuated, token::Comma};

pub fn parse_named_fields(input: &DeriveInput) -> Result<&Punctuated<Field, Comma>, TokenStream> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => Ok(&named.named),
            _ => Err(quote! { compile_error!("Builder supports only named fields") }),
        },
        _ => Err(quote! { compile_error!("Builder supports only structs") }),
    }
}
