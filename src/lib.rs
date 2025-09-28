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
    pub(crate) mod getters;
    pub(crate) mod init;
    pub(crate) mod setters;
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
    generators::{accessors, builder, init},
    utils::parser::parse_attribute,
};

#[proc_macro_derive(Test)]
pub fn test_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{input:?}");
    quote! {}.into()
}

#[proc_macro_derive(
    Accessors,
    attributes(
        accessors, getters, setters, validator, size, content, date, default, email, regex
    )
)]
#[proc_macro_error]
pub fn accessors_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generated = accessors::generate(&input);
    quote! { #generated }.into()
}

#[proc_macro_derive(
    Getters,
    attributes(getters, validator, size, content, date, default, email, regex)
)]
#[proc_macro_error]
pub fn getters_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generated = quote! {};
    quote! { #generated }.into()
}

#[proc_macro_derive(Setters, attributes(setters))]
#[proc_macro_error]
pub fn setters_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generated = quote! {};
    quote! { #generated }.into()
}

#[proc_macro_derive(
    Builder,
    attributes(builder, validator, size, content, date, default, email, regex)
)]
#[proc_macro_error]
pub fn builder_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generated = builder::generate(&input);
    quote! { #generated }.into()
}

#[proc_macro_derive(
    Init,
    attributes(init, validator, size, content, date, default, email, regex)
)]
#[proc_macro_error]
pub fn init_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generated = init::generate(&input);
    quote! { #generated }.into()
}
