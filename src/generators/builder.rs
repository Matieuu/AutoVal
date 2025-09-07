use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

use crate::utils::{checker::has_token, parser::parse_named_fields};

pub fn generate(input: &DeriveInput) -> TokenStream {
    if !has_token(&input.attrs, "autoval", "builder") {
        return TokenStream::new();
    }

    let input_ident = &input.ident;
    let builder_ident = Ident::new(&format!("{input_ident}Builder"), input_ident.span());
    let fields = parse_named_fields(input);

    let mut builder_fields: Vec<TokenStream> = Vec::new();
    let mut builder_setters: Vec<TokenStream> = Vec::new();
    let mut builder_init: Vec<TokenStream> = Vec::new();
    let mut build_fields: Vec<TokenStream> = Vec::new();

    for field in fields {
        let field_ident = &field.ident;
        let field_type = &field.ty;

        builder_fields.push(quote! {
            #field_ident: ::core::option::Option<#field_type>
        });

        builder_setters.push(quote! {
            pub fn #field_ident(mut self, value: #field_type) -> Self {
                self.#field_ident = ::core::option::Option::Some(value);
                self
            }
        });

        builder_init.push(quote! {
            #field_ident: ::core::option::Option::None
        });

        build_fields.push(quote! {
            #field_ident: self.#field_ident.ok_or(concat!("Field `", stringify!(#field_ident), "` is missing"))?
        });
    }

    quote! {
        impl #input_ident {
            pub fn builder() -> #builder_ident {
                ::core::default::Default::default()
            }
        }

        pub struct #builder_ident {
            #( #builder_fields, )*
        }

        impl #builder_ident {
            pub fn build(self) -> ::core::result::Result::<#input_ident, ::std::string::String> {
                ::core::result::Result::Ok(
                    #input_ident {
                        #( #build_fields, )*
                    }
                )
            }

            #( #builder_setters )*
        }

        impl ::core::default::Default for #builder_ident {
            fn default() -> Self {
                Self { #( #builder_init, )* }
            }
        }
    }
}
