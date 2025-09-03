use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::utils::{default_pub, parse_visibility_from};

pub fn generate(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{name}Builder"), name.span());
    let struct_vis = default_pub(parse_visibility_from(&input.attrs, "builder").unwrap());

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            _ => unimplemented!("Builder supports only named fields"),
        },
        _ => unimplemented!("Builder supports only structs"),
    };

    let builder_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! { #ident: ::core::option::Option<#ty> }
    });

    let builder_init = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! { #ident: ::core::option::Option::None }
    });

    let builder_setters = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #struct_vis fn #ident(mut self, #ident: #ty) -> Self {
                self.#ident = ::core::option::Option::Some(#ident);
                self
            }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! {
            #ident: self.#ident.ok_or(concat!("Field `", stringify!(#ident), "` is missing"))?
        }
    });

    quote! {
        pub struct #builder_name {
            #( #builder_fields, )*
        }

        impl #builder_name {
            #struct_vis fn build(self) -> ::core::result::Result<#name, ::std::string::String> {
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
            #struct_vis fn builder() -> #builder_name {
                ::core::default::Default::default()
            }
        }
    }
}
