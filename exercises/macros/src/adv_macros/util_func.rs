use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::__private::ext::RepToTokensExt;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, FnArg, ImplItem, Item, ItemFn, ItemImpl, ItemMod,
    LitStr, Pat, Path, Token,
};
use syn::{Attribute, Expr};

// type AttributeArgs = Punctuated<Expr, Token![,]>;
pub(crate) fn calculate_sum(input: TokenStream) -> TokenStream {
    // for multi params, separated by commas
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    let iter = args.iter();
    let expanded = quote! {
        0 #( + #iter )*
    };
    TokenStream::from(expanded)
}

fn get_before_after_paths(args: Punctuated<Expr, Token![,]>) -> (Vec<Path>, Vec<Path>) {
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
                                fn_args.iter().for_each(|itm| {
                                    if let Expr::Path(path) = itm {
                                        befores.push(path.path.clone())
                                    }
                                });
                            }
                            "after" => {
                                fn_args.iter().for_each(|itm| {
                                    if let Expr::Path(path) = itm {
                                        afters.push(path.path.clone())
                                    }
                                });
                            }
                            _ => {}
                        }
                        if ident == "before" {}
                    }
                }
            }
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
fn with_wrap_fn(item_fn: ItemFn, before: Vec<Path>, after: Vec<Path>) -> proc_macro2::TokenStream {
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

fn with_wrap_mod(
    mut item_mod: ItemMod,
    before: Vec<Path>,
    after: Vec<Path>,
) -> proc_macro2::TokenStream {
    if let Some((_, ref mut items)) = &mut item_mod.content {
        for item in items.iter_mut() {
            if let Item::Fn(item_fn) = item {
                let with_existed = item_fn
                    .attrs
                    .iter()
                    .any(|item| item.path().segments.last().unwrap().ident.eq("utils::with"));
                if !with_existed {
                    let wrapped_fn = with_wrap_fn(item_fn.clone(), before.clone(), after.clone());
                    *item_fn = syn::parse2(wrapped_fn).unwrap();
                }
            }
        }
    }
    quote! { #item_mod }
}

fn with_wrap_struct_impl(
    mut item_impl: ItemImpl,
    before: Vec<Path>,
    after: Vec<Path>,
) -> proc_macro2::TokenStream {
    for item in item_impl.items.iter_mut() {
        if let ImplItem::Fn(item_method) = item {
            let item_fn = ItemFn {
                attrs: item_method.attrs.clone(),
                vis: item_method.vis.clone(),
                sig: item_method.sig.clone(),
                block: Box::new(item_method.block.clone()),
            };
            let wrap_method = with_wrap_fn(item_fn, before.clone(), after.clone());
            *item_method = syn::parse2(wrap_method).unwrap();
        }
    }
    quote! { #item_impl }
}

pub(crate) fn create_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Expr, Token![,]>::parse_terminated);
    // extract before and after interceptor
    let (before, after) = get_before_after_paths(args);
    let input = parse_macro_input!(item as Item);

    let output = match input {
        Item::Fn(item_fn) => with_wrap_fn(item_fn, before.clone(), after.clone()),
        Item::Mod(item_mod) => with_wrap_mod(item_mod, before.clone(), after.clone()),
        Item::Impl(item_impl) => with_wrap_struct_impl(item_impl, before.clone(), after.clone()),
        _ => panic!("with attribute supports fn, mod or struct only"),
    };

    TokenStream::from(output)
}

fn crud_get_tablename_pkkeys(ident_name: String, attrs: &Vec<Attribute>) -> (String, Vec<String>) {
    let mut tbl_name = ident_name;
    let mut pk_fields: Vec<String> = vec![];
    // crud(table_name = "test_rec", primary_key(id, name))
    // let attrs = &input.attrs;
    for attr in attrs {
        // find `crud` attr
        if attr.path().is_ident("crud") {
            // find `table_name` in `crud`
            let _ = attr.parse_nested_meta(|crud_meta| {
                if crud_meta.path.is_ident("table_name") {
                    if let Ok(meta_value) = crud_meta.value() {
                        if let Ok(right_value) = meta_value.parse::<syn::LitStr>() {
                            tbl_name = right_value.value();
                        }
                    }
                    return Ok(());
                }
                if crud_meta.path.is_ident("primary_key") {
                    let _ = crud_meta.parse_nested_meta(|primary_key_meta| {
                        if let Some(ident) = primary_key_meta.path.get_ident() {
                            pk_fields.push(ident.to_string());
                            Ok(())
                        } else {
                            Err(primary_key_meta.error("unrecognized repr"))
                        }
                    });
                    return Ok(());
                }
                // Err(crud_meta.error("unrecognized repr"))
                Ok(())
            });
        }
    }
    (tbl_name, pk_fields)
}

fn crud_get_fields(input_data: &Data) -> (Vec<String>, Vec<Ident>) {
    let mut field_names: Vec<String> = vec![];
    let mut field_idents: Vec<Ident> = vec![];

    if let Data::Struct(data_struct) = input_data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in fields_named.named.iter() {
                let ident = field.ident.as_ref().unwrap();
                let mut col_name = ident.to_string();

                for attr in &field.attrs {
                    // find `column` in #[column(name="")]
                    if attr.path().is_ident("column") {
                        let _ = attr.parse_nested_meta(|column_meta| {
                            if column_meta.path.is_ident("name") {
                                if let Ok(meta_value) = column_meta.value() {
                                    if let Ok(right_value) = meta_value.parse::<syn::LitStr>() {
                                        col_name = right_value.value();
                                    }
                                }
                            }
                            Ok(())
                        });
                    }
                }
                field_names.push(col_name);
                field_idents.push(ident.clone());
            }
        }
    }

    (field_names, field_idents)
}

pub(crate) fn create_crud(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let input_attrs = input.attrs;
    let (table_name, pk_fields) = crud_get_tablename_pkkeys(struct_name.to_string(), &input_attrs);

    let input_data = input.data;
    let (field_names, field_idents) = crud_get_fields(&input_data);

    // prepare template

    let insert_params = (1..=field_idents.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<String>>();
    let insert_params_str = insert_params.join(",");
    let pk_fields_str = pk_fields.join(",");

    let mut get_ident_from_result: Vec<proc_macro2::TokenStream> = vec![];
    for (field_name, field_ident) in field_names.iter().zip(field_idents) {
        let tmp_ident = quote! {row.get::<_,_>(#field_name) };
        get_ident_from_result.push(tmp_ident);
    }
    // let get_ident_from_result_str = get_ident_from_result.join(", ");
    let col_list_str = LitStr::new(field_names.join(", ").as_str(), Span::call_site());
    let table_name = LitStr::new(table_name.as_str(), Span::call_site());
    let pk_fields_str = LitStr::new(pk_fields.join(",").as_str(), Span::call_site());
    let find_all_fn = quote! {
        pub async fn find_all(page_no: u32, page_size: u32) -> AppResult<Vec<#struct_name>> {
            use tokio_postgres::types::ToSql;
            use chrono::Local;
            use chrono::NaiveDateTime;
            use tokio::pin;
            use tokio_stream::StreamExt;
            use web::persistence::common::get_async_connection;

            let conn = get_async_connection().await?;
            let mut result: Vec<#struct_name> = vec![];

            let offset_val = (page_no * page_size) as i64;
            let page_size = page_size as i64;
            let sql = format!("select {} from {} order by {} desc limit $2 offset $1",
                #col_list_str, #table_name, #pk_fields_str);
            let rows = conn.query_raw(&sql, &[offset_val, page_size]).await?;
            pin!(rows);
            while let Some(row) = rows.next().await {
                if let Ok(row) = row {
                    let rec = #struct_name::new(
                        #(#get_ident_from_result),*
                    );
                    result.push(rec);
                }
            }
            Ok(result)
        }
    };

    let find_by_id = quote! {};

    let insert_fn = quote! {
        pub fn insert(val: &#struct_name) -> AppResult<()> {
            let table_name = #table_name.to_string();
            let primary_keys = [#(#pk_fields),*];
            Ok(())
        }
    };

    let update_fn = quote! {};

    let delete_fn = quote! {};

    let output = quote! {
        #find_all_fn
        #find_by_id
        #insert_fn
        #update_fn
        #delete_fn
    };

    TokenStream::from(output)
    // output.into()
}
