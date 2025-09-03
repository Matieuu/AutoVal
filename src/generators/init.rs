use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::utils::{default_pub, is_option, parse_visibility_from};

pub fn generate(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let struct_vis = default_pub(parse_visibility_from(&input.attrs, "init").unwrap());

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => unimplemented!("Constructors supports only structs"),
    };

    let mut req_args: Vec<TokenStream> = Vec::new();
    let mut req_init: Vec<TokenStream> = Vec::new();
    let mut opt_init: Vec<TokenStream> = Vec::new();

    for field in fields {
        if is_option(&field.ty) {
            let ident = &field.ident;
            opt_init.push(quote! { #ident: None });
        } else {
            let ident = &field.ident;
            let ty = &field.ty;
            req_args.push(quote! { #ident: #ty });
            req_init.push(quote! { #ident });
        }
    }

    let req_args = req_args.iter();
    let req_init = req_init.iter();
    let opt_init = opt_init.iter();

    quote! {
        impl #name {
            #struct_vis fn new( #( #req_args ),* ) -> Self {
                Self {
                    #( #req_init, )*
                    #( #opt_init, )*
                }
            }
        }
    }
}
