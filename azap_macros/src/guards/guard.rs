use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Expr};

pub fn guards_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _guards = parse_macro_input!(attr with Punctuated::<Expr, Comma>::parse_terminated);
    let item_ts = proc_macro2::TokenStream::from(item);

    // Attach metadata as a hidden attribute
    let expanded = quote! {
        #item_ts
    };

    expanded.into()
}
