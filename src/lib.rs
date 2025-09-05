#![warn(clippy::all, clippy::pedantic)]
#![forbid(
    unsafe_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ok_expect,
    clippy::err_expect
)]
#![deny(
    unsafe_code,
    unused_must_use,
    unreachable_patterns,
    unused_variables,
    rust_2018_idioms
)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_mut, unused_variables)
)]
#![cfg_attr(not(debug_assertions), deny(debug_assertions))]

pub(crate) mod generators {
    pub(crate) mod accessors;
    pub(crate) mod builder;
}
pub(crate) mod utils {
    pub(crate) mod checker;
    pub(crate) mod parser;
}

use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::{
    generators::{accessors, builder},
    utils::checker::has_attribute,
};

#[proc_macro_derive(
    Autoval,
    attributes(autoval, size, content, date, default, email, regex)
)]
#[proc_macro_error]
pub fn autoval_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !has_attribute(&input.attrs, "autoval") {
        let input_ident = &input.ident;
        abort!(
            input_ident,
            "Struct {} with derive macro Autoval doesn't have autoval attribute",
            input_ident
        );
    }

    let accessors_generated = accessors::generate(&input);
    let builder_generated = builder::generate(&input);

    quote! {
        #accessors_generated
        #builder_generated
    }
    .into()
}
