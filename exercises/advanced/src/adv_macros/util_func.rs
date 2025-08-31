use log::info;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Token};

pub(crate) fn sum(input: TokenStream) -> TokenStream {
    info!("{:?}", input);
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    let iter = args.iter();
    let expanded = quote! {
        0 #( + #iter )*
    };
    TokenStream::from(expanded)
}
