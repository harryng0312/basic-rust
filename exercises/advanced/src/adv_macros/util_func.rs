use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, AttributeArgs, Expr, FnArg, ItemFn, Meta, NestedMeta, Pat,
    Path, Token,
};

pub(crate) fn calculate_sum(input: TokenStream) -> TokenStream {
    // for multi params, separated by commas
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    let iter = args.iter();
    let expanded = quote! {
        0 #( + #iter )*
    };
    TokenStream::from(expanded)
}

fn get_before_after_paths(args: AttributeArgs) -> (Vec<Path>, Vec<Path>) {
    let mut befores: Vec<Path> = vec![];
    let mut afters: Vec<Path> = vec![];
    for (idx, arg) in args.iter().enumerate() {
        match arg {
            NestedMeta::Meta(Meta::List(list)) => {
                let ident = &list.path.segments.last().unwrap().ident;
                if list.nested.len() >= 1 {
                    if ident == "before" {
                        list.nested.iter().for_each(|m| match m {
                            NestedMeta::Meta(Meta::Path(path)) => {
                                befores.push(path.clone());
                            }
                            _ => (),
                        });
                    } else if ident == "after" {
                        list.nested.iter().for_each(|m| match m {
                            NestedMeta::Meta(Meta::Path(path)) => {
                                afters.push(path.clone());
                            }
                            _ => (),
                        });
                    }
                }
            }
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                eprintln!("Can not parse argument: {}", nv.path.get_ident().unwrap());
            }
            _ => {
                eprintln!("Unexpected argument: {}", idx);
            }
        };
    }
    (befores, afters)
}
pub(crate) fn create_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    // extract before and after interceptor
    let (before, after) = get_before_after_paths(args);

    // get original fn
    let fn_sig = &input_fn.sig;
    let fn_name = &fn_sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_attrs = &input_fn.attrs;
    let fn_block = &input_fn.block;

    // extract params
    let fn_params = fn_sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat) => match &*pat.pat {
                Pat::Ident(pat) => Some(pat.clone().ident),
                _ => None,
            },
            FnArg::Receiver(_) => None,
            // FnArg::Typed(pat) => Some(pat.pat.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    // build &[&dyn Any]
    let param_refs = fn_params
        .iter()
        // .map(|param| parse_quote! { &#param })
        // .collect::<Vec<Expr>>();
        // .map(|param| quote! { &#param as &dyn std::any:Any })
        .map(|param| quote! { &#param })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let param_refs = quote! { #(#param_refs),* };

    let expanded = quote! {
        #fn_vis #(#fn_attrs)* #fn_sig {
            // #( #befores(stringify!(#fn_name), &[ #(&#params_refs as &dyn std::any::Any ),* ] ); )*
            // #( #befores(stringify!(#fn_name), &[#(#params_refs),*] )*; )*
            #(#before(stringify!(#fn_name), &[ #param_refs ]); )*
            let __result = (|| #fn_block )();
            // #( #after(stringify!(#fn_name), &__result as &dyn std::any::Any,
            //     &[ #( & #params_refs as &dyn std::any::Any ),* ] ); )*
            #(#after(stringify!(#fn_name), &__result, &[ #param_refs ]); )*
            __result
        }
    };
    TokenStream::from(expanded)
}
