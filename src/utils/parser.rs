use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, punctuated::Punctuated, token::Comma};

pub fn parse_named_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => abort!(input, "Builder supports only named fields"),
        },
        _ => abort!(input, "Builder supports only structs"),
    }
}
