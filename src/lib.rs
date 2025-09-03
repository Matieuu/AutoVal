pub(crate) mod generators {
    pub(crate) mod builder;
    pub(crate) mod getter;
    pub(crate) mod init;
    pub(crate) mod setter;
}
pub(crate) mod utils;
pub(crate) mod validators {}

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use crate::generators::{builder, getter, init, setter};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder::generate(&input).into()
}

#[proc_macro_derive(Setter, attributes(setter))]
pub fn setter_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    setter::generate(&input).into()
}

#[proc_macro_derive(Getter, attributes(getter, Setter))]
pub fn getter_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    getter::generate(&input).into()
}

#[proc_macro_derive(Init, attributes(init))]
pub fn init_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    init::generate(&input).into()
}
