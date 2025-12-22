use proc_macro::TokenStream;

use crate::{
    guards::{guard::guards_macro, register::register_guard_macro},
    route::route_macro,
};

mod guards;
mod route;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, input: TokenStream) -> TokenStream {
    route_macro("get", attr, input)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, input: TokenStream) -> TokenStream {
    route_macro("post", attr, input)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, input: TokenStream) -> TokenStream {
    route_macro("put", attr, input)
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, input: TokenStream) -> TokenStream {
    route_macro("patch", attr, input)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, input: TokenStream) -> TokenStream {
    route_macro("delete", attr, input)
}

#[proc_macro_attribute]
pub fn guards(attr: TokenStream, input: TokenStream) -> TokenStream {
    guards_macro(attr, input)
}

#[proc_macro_attribute]
pub fn register_guard(attr: TokenStream, input: TokenStream) -> TokenStream {
    register_guard_macro(attr, input)
}
