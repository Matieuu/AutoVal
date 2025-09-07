use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, Ident, Lit, LitStr};

use crate::{
    parse_attribute,
    utils::{
        checker::has_token,
        parser::{parse_named_fields, parse_optional_field},
    },
};

pub fn generate(input: &DeriveInput) -> TokenStream {
    if !has_token(&input.attrs, "autoval", "init") {
        return TokenStream::new();
    }

    let input_ident = &input.ident;
    let fields = parse_named_fields(input);

    let mut required_fields: Vec<TokenStream> = Vec::new();
    let mut required_init: Vec<TokenStream> = Vec::new();
    let mut defaults_init: Vec<TokenStream> = Vec::new();
    let mut optional_init: Vec<TokenStream> = Vec::new();

    for field in fields {
        let field_ident = &field.ident;
        let field_type = &field.ty;
        let (parsed_type, is_optional) = parse_optional_field(field);
        let default_attr = parse_attribute(&field.attrs, "default");

        if let Some(attr) = default_attr {
            if let Ok(Lit::Str(lit)) = attr.parse_args() {
                let expr: Result<Expr, _> = lit.parse();
                if let Ok(expr) = expr {
                    defaults_init.push(quote! {
                        #field_ident: #expr
                    });
                } else {
                    abort!(
                        field,
                        "Attribute `default` in field `{}` contains invalid expression",
                        stringify!(field.ident)
                    );
                }
            } else {
                abort!(
                    field,
                    "Field `{}` has attribute `default` but has no default value",
                    stringify!(field.ident)
                );
            }
        } else if is_optional {
            optional_init.push(quote! {
                #field_ident: ::core::option::Option::None
            });
        } else {
            required_fields.push(quote! {
                #field_ident: #field_type
            });

            required_init.push(quote! {
                #field_ident: #field_ident
            });
        }
    }

    quote! {
        impl #input_ident {
            pub fn new(
                #( #required_fields, )*
            ) -> Self {
                Self {
                    #( #required_init, )*
                    #( #defaults_init, )*
                    #( #optional_init, )*
                }
            }
        }
    }
}
