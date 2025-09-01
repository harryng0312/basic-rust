use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, Expr, FnArg, ImplItem, Item, ItemFn, ItemImpl, ItemMod, Pat, Path, Token,
};

type AttributeArgs = Punctuated<Expr, Token![,]>;
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
            // NestedMeta::Meta(Meta::List(list)) => {
            Expr::Call(call) => {
                if let Expr::Path(ref fn_call) = *call.func {
                    let fn_args = &call.args;
                    if !fn_args.is_empty() {
                        let ident = &fn_call.path.segments.last().unwrap().ident;
                        match ident.to_string().as_str() {
                            "before" => {
                                fn_args.iter().for_each(|itm| match itm {
                                    Expr::Path(path) => befores.push(path.path.clone()),
                                    _ => {}
                                });
                            }
                            "after" => {
                                fn_args.iter().for_each(|itm| match itm {
                                    Expr::Path(path) => afters.push(path.path.clone()),
                                    _ => {}
                                });
                            }
                            _ => {}
                        }
                        if ident == "before" {}
                    }
                }
                // let ident = call.func.segments.last().unwrap().ident;
                // if list.nested.len() >= 1 {
                //     if ident == "before" {
                //         list.nested.iter().for_each(|m| match m {
                //             NestedMeta::Meta(Meta::Path(path)) => {
                //                 befores.push(path.clone());
                //             }
                //             _ => (),
                //         });
                //     } else if ident == "after" {
                //         list.nested.iter().for_each(|m| match m {
                //             NestedMeta::Meta(Meta::Path(path)) => {
                //                 afters.push(path.clone());
                //             }
                //             _ => (),
                //         });
                //     }
                // }
            }
            // NestedMeta::Meta(Meta::NameValue(nv)) => {
            //     eprintln!("Can not parse argument: {}", nv.path.get_ident().unwrap());
            // }
            Expr::MethodCall(call) => {
                let ident = &call.method;
                println!("Can not parse argument: MethodCall to {}", ident);
            }
            Expr::Assign(assign) => {
                if let Expr::Path(ref fn_call) = *assign.left {
                    println!(
                        "Can not parse argument: Assign to {}",
                        fn_call.path.segments.last().unwrap().ident
                    );
                }
            }
            _ => {
                eprintln!("Unexpected argument: {}", idx);
            }
        };
    }
    (befores, afters)
}
pub(crate) fn wrap_fn(
    item_fn: ItemFn,
    before: Vec<Path>,
    after: Vec<Path>,
) -> proc_macro2::TokenStream {
    // get original fn
    let fn_sig = &item_fn.sig;
    let fn_name = &fn_sig.ident;
    let fn_vis = &item_fn.vis;
    let fn_attrs = &item_fn.attrs;
    let fn_block = &item_fn.block;
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
        // .collect::<Vec<Expr>>();
        // .map(|param| quote! { &#param as &dyn std::any:Any })
        .map(|param| quote! { &#param })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let param_refs = quote! { #(#param_refs),* };
    let output = match fn_sig.asyncness {
        Some(_) => quote! {
            #fn_vis #(#fn_attrs)* #fn_sig {
                #(#before(stringify!(#fn_name), &[ #param_refs ]); )*
                let __result = (async || #fn_block )().await;
                #(#after(stringify!(#fn_name), &__result, &[ #param_refs ]); )*
                __result
            }
        },
        None => quote! {
            #fn_vis #(#fn_attrs)* #fn_sig {
                #(#before(stringify!(#fn_name), &[ #param_refs ]); )*
                let __result = (|| #fn_block )();
                #(#after(stringify!(#fn_name), &__result, &[ #param_refs ]); )*
                __result
            }
        },
    };

    output
}

pub(crate) fn wrap_mod(
    mut item_mod: ItemMod,
    before: Vec<Path>,
    after: Vec<Path>,
) -> proc_macro2::TokenStream {
    if let Some((_, ref mut items)) = &mut item_mod.content {
        for item in items.iter_mut() {
            match item {
                Item::Fn(item_fn) => {
                    let with_existed = item_fn.attrs.iter().any(|item| {
                        item.path()
                            .segments
                            .last()
                            .unwrap()
                            .ident
                            .eq("macros::with")
                    });
                    if !with_existed {
                        let wrapped_fn = wrap_fn(item_fn.clone(), before.clone(), after.clone());
                        *item_fn = syn::parse2(wrapped_fn).unwrap();
                    }
                }
                _ => {}
            }
        }
    }
    quote! { #item_mod }
}

pub(crate) fn wrap_struct_impl(
    mut item_impl: ItemImpl,
    before: Vec<Path>,
    after: Vec<Path>,
) -> proc_macro2::TokenStream {
    for item in item_impl.items.iter_mut() {
        match item {
            ImplItem::Fn(item_method) => {
                let item_fn = ItemFn {
                    attrs: item_method.attrs.clone(),
                    vis: item_method.vis.clone(),
                    sig: item_method.sig.clone(),
                    block: Box::new(item_method.block.clone()),
                };
                let wrap_method = wrap_fn(item_fn, before.clone(), after.clone());
                *item_method = syn::parse2(wrap_method).unwrap();
            }
            _ => {}
        }
    }
    quote! { #item_impl }
}

pub(crate) fn create_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with AttributeArgs::parse_terminated);
    // extract before and after interceptor
    let (before, after) = get_before_after_paths(args);
    let input = parse_macro_input!(item as Item);

    let output = match input {
        Item::Fn(item_fn) => wrap_fn(item_fn, before.clone(), after.clone()),
        Item::Mod(item_mod) => wrap_mod(item_mod, before.clone(), after.clone()),
        Item::Impl(item_impl) => wrap_struct_impl(item_impl, before.clone(), after.clone()),
        _ => panic!("with attribute supports fn, mod or struct only"),
    };

    TokenStream::from(output)
}
