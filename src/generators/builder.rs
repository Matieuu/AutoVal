use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident};

use crate::{
    helpers::is_validated,
    utils::{default_pub, parse_visibility_from},
    validators::validate,
};

pub fn generate(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let builder_name = Ident::new(&format!("{name}Builder"), name.span());
    let struct_vis = default_pub(parse_visibility_from(&input.attrs, "builder").unwrap());

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => unimplemented!("Builder supports only named fields"),
        },
        _ => unimplemented!("Builder supports only structs"),
    };

    let mut builder_fields: Vec<TokenStream> = Vec::new();
    let mut build_fields: Vec<TokenStream> = Vec::new();
    let mut builder_setters: Vec<TokenStream> = Vec::new();
    let mut builder_init: Vec<TokenStream> = Vec::new();
    let mut builder_validators: Vec<TokenStream> = Vec::new();
    let mut validators_funcs: Vec<TokenStream> = Vec::new();

    for field in fields {
        let ident = &field.ident;
        let ty = &field.ty;

        builder_fields.push(quote! { #ident: ::core::option::Option<#ty> });
        build_fields.push(quote! {
            #ident: self.#ident.ok_or(concat!("Field `", stringify!(#ident), "` is missing"))?
        });
        builder_init.push(quote! { #ident: ::core::option::Option::None });
        builder_setters.push(quote! {
            #struct_vis fn #ident(mut self, #ident: #ty) -> Self {
                self.#ident = ::core::option::Option::Some(#ident);
                self
            }
        });

        if is_validated(input) {
            validators_funcs.push(validate::check_field(field));
            let func_name = format_ident!("validate_{}", ident.as_ref().unwrap());
            builder_validators.push(quote! {
                if let ::core::option::Option::Some(ref value) = self.#ident {
                    #name::#func_name(value)?;
                }
            });
        }
    }

    quote! {
        #struct_vis struct #builder_name {
            #( #builder_fields, )*
        }

        impl #builder_name {
            #struct_vis fn build(self) -> ::core::result::Result<#name, ::std::string::String> {
                #( #builder_validators )*

                let out = #name {
                    #( #build_fields, )*
                };
                ::core::result::Result::Ok(out)
            }

            #( #builder_setters )*
        }

        impl ::core::default::Default for #builder_name {
            fn default() -> Self {
                Self { #( #builder_init, )* }
            }
        }

        impl #name {
            #( #validators_funcs )*

            #struct_vis fn builder() -> #builder_name {
                ::core::default::Default::default()
            }
        }
    }
}
