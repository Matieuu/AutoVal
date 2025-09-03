use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, spanned::Spanned};

use crate::utils::{default_pub, parse_visibility_from};

pub fn generate(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let struct_vis = default_pub(parse_visibility_from(&input.attrs, "getter").unwrap());
    let mut methods = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                let fname = &field.ident;
                let setter_name =
                    Ident::new(&format!("set_{}", &fname.clone().unwrap()), fname.span());
                let fty = &field.ty;

                let fvis = parse_visibility_from(&field.attrs, "setter")
                    .unwrap()
                    .unwrap_or_else(|| struct_vis.clone());

                methods.push(quote! {
                    #fvis fn #setter_name(&mut self, value: #fty) {
                        self.#fname = value;
                    }
                });
            }
        } else {
            unimplemented!("Setter supports only named fields")
        }
    } else {
        unimplemented!("Setter supports only structs")
    }

    quote! {
        impl #name {
            #(#methods)*
        }
    }
}
