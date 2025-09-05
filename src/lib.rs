#![forbid(unsafe_code)]
#![deny(
    unsafe_code,
    unused_must_use,
    unreachable_patterns,
    unused_variables,
    rust_2018_idioms
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_mut, unused_variables)
)]
#![cfg_attr(not(debug_assertions), deny(debug_assertions))]

pub(crate) mod generators {
    pub(crate) mod builder;
}
pub(crate) mod utils {
    pub(crate) mod checker;
    pub(crate) mod parser;
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::{generators::builder, utils::checker::has_attribute};

#[proc_macro_derive(
    Autoval,
    attributes(autoval, size, content, date, default, email, regex)
)]
pub fn autoval_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !has_attribute(&input.attrs, "autoval") {
        return TokenStream::new();
    }

    let builder_generated = builder::generate(&input);

    quote! {
        #builder_generated
    }
    .into()
}
