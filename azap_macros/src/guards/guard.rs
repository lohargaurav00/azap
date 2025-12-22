use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Expr};

pub fn guards_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let guards = parse_macro_input!(attr with Punctuated::<Expr, Comma>::parse_terminated);
    let item_token = proc_macro2::TokenStream::from(item);

    for guard in &guards {
        match guard {
            Expr::Path(_) => {
                // auth
            }
            Expr::Call(call) => {
                // rate_limit(100, 5)
                let func = &call.func;
                let args = &call.args;
                dbg!(func, args);
            }
            _ => {
                return syn::Error::new_spanned(guard, "invalid guard syntax")
                    .to_compile_error()
                    .into();
            }
        }
    }

    dbg!(guards);
    let expand = quote! {
        #item_token
    };

    expand.into()
}
