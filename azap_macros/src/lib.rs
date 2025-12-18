use proc_macro::TokenStream;

use crate::route::route_macro;

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
