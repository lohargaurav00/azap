use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

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

fn route_macro(method: &str, attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let path = parse_macro_input!(attr as LitStr);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    let method_upper = method.to_uppercase();
    let metadata_const = quote::format_ident!(
        "__AZAP_ROUTE_{}_{}",
        method_upper,
        fn_name.to_string().to_uppercase()
    );

    let expand = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #fn_block
        }


        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        pub const #metadata_const: azap::RouteMetaData = azap::RouteMetaData {
            method: #method,
            path: #path,
            handler_name: stringify!(#fn_name),
            module: module_path!(),
            file: file!()
        };
    };

    TokenStream::from(expand)
}
