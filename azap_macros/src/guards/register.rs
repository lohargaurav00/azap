use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, ItemFn, LitStr, Token,
};

#[derive(Debug, PartialEq, Clone)]
enum GuardType {
    FromFn,
    FromFnWithState,
    Layer,
}

impl GuardType {
    fn from_str(s: &str) -> syn::Result<Self> {
        match s {
            "fn" => Ok(GuardType::FromFn),
            "fn_with_state" => Ok(GuardType::FromFnWithState),
            "layer" => Ok(GuardType::Layer),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown guard type: '{}'. Expected 'fn', 'fn_with_state', or 'layer'",
                    s
                ),
            )),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::FromFn => "fn".to_owned(),
            Self::FromFnWithState => "fn_with_state".to_owned(),
            Self::Layer => "layer".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct RegisterGuardArgs {
    guard_type: GuardType,
}

impl Parse for RegisterGuardArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        if key != "guard_type" {
            return Err(syn::Error::new_spanned(key, "Expected 'guard_type'"));
        };

        let _: Token![=] = input.parse()?;

        let value: LitStr = input.parse()?;

        let guard_type = GuardType::from_str(&value.value())?;

        Ok(RegisterGuardArgs { guard_type })
    }
}

pub(crate) fn register_guard_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let parsed_attr = parse_macro_input!(attr as RegisterGuardArgs);

    let guard_type = parsed_attr.guard_type.to_string();
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
                #[register_guard(\"guard_type = {}\")]\n\
                pub async fn {}(...) {{ ... }}",
                fn_name, guard_type, fn_name
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
                #[register_guard(\"guard_type = {}\")]\n\
                pub async fn {}(...) {{ ... }}",
                fn_name, guard_type, fn_name
            ),
        )
        .to_compile_error()
        .into();
    }

    let expand = quote! {
       // guard_type = #guard_type
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #fn_block
        }


    };

    expand.into()
}
