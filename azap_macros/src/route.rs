use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

pub(crate) fn route_macro(method: &str, attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let path = parse_macro_input!(attr as LitStr);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    if fn_sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &fn_sig.fn_token,
            format!(
                "Route handler '{}' must be async.\n\
                Help: Add 'async' keyword before 'fn':\n\
                #[{}(\"{}\")]\n\
                pub async fn {}(...) {{ ... }}",
                fn_name,
                method,
                path.value(),
                fn_name
            ),
        )
        .to_compile_error()
        .into();
    }

    if !matches!(fn_vis, syn::Visibility::Public(_)) {
        return syn::Error::new_spanned(
            &fn_sig.fn_token,
            format!(
                "Route handler '{}' must be public.\n\
                Help: Add 'pub' keyword:\n\
                #[{}(\"{}\")]\n\
                pub async fn {}(...) {{ ... }}",
                fn_name,
                method,
                path.value(),
                fn_name
            ),
        )
        .to_compile_error()
        .into();
    }

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
            file: file!(),
            line: line!(),
            column: column!()
        };
    };

    TokenStream::from(expand)
}
