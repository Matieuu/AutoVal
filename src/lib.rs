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

pub(crate) mod generators;
pub(crate) mod helpers;
pub(crate) mod utils;
pub(crate) mod validators;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use crate::generators::{builder, getter, init, setter};

#[proc_macro_derive(Builder, attributes(builder, validate, size))]
pub fn builder_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // println!("{input:?}");
    builder::generate(&input).into()
}

#[proc_macro_derive(Setter, attributes(setter, validate, size))]
pub fn setter_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    setter::generate(&input).into()
}

#[proc_macro_derive(Getter, attributes(getter))]
pub fn getter_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getter::generate(&input).into()
}

#[proc_macro_derive(Init, attributes(init, validate, size))]
pub fn init_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    init::generate(&input).into()
}

// #[proc_macro_derive(
//     Validate,
//     attributes(
//         max,
//         // min, future, past, negative, positive, capitalize, lowercase, uppercase, pattern,
//         // email, notblank, notempty, some, none, default
//     )
// )]

// #[proc_macro_attribute]
// pub fn validate(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     validator::generate(&input).into()
// }

// // dummy attributes for validate
// #[proc_macro_attribute]
// pub fn size(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }
