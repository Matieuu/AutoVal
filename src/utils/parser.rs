use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, GenericArgument, Ident, PathArguments, Type,
    punctuated::Punctuated, token::Comma,
};

pub fn parse_named_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => abort!(input, "Builder supports only named fields"),
        },
        _ => abort!(input, "Builder supports only structs"),
    }
}

pub fn parse_optional_field(field: &Field) -> (&Type, bool) {
    if let Type::Path(typepath) = &field.ty {
        if let Some(seg) = typepath.path.segments.last() {
            // eprintln!("{seg:?}");
            if seg.ident == "Option" {
                // is_optional = true;
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.last() {
                        return (inner_ty, true);
                    }
                }
            }
        }
    }
    (&field.ty, false)
}

pub fn parse_attribute<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident(name))
}
