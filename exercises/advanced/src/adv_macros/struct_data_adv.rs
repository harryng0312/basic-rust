use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Fields, ItemStruct, Lit, Meta, NestedMeta, Path};

pub(crate) fn create_record(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);

    let mut derives: Vec<Path> = vec![];
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::Path(path)) => {
                // ex: #[record(SomeAttr)]
                // println!("Found Path: {}", path.into_token_stream());
                derives.push(path);
            }
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                // ex: #[record(extra = "yes")]
                // let ident = nv.path.into_token_stream().to_string();
                // let lit = nv.lit.into_token_stream().to_string();
                // println!("Found NameValue: {} = {}", ident, lit);
                if nv.path.is_ident("derive") {
                    if let Lit::Str(litstr) = &nv.lit {
                        let tokens_str = litstr.value();
                        for d in tokens_str.split(',') {
                            let d = d.trim();
                            if !d.is_empty() {
                                let path: Path = syn::parse_str(d).unwrap();
                                derives.push(path);
                            }
                        }
                    }
                }
            }
            NestedMeta::Meta(Meta::List(list)) => {
                // ex: #[record(derive(Debug, Clone))]
                // let ident = list.path.into_token_stream().to_string();
                // println!("Found List: {}", ident);

                for nested2 in list.nested {
                    match nested2 {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            // println!("  List item Path: {}", path.clone().into_token_stream());
                            derives.push(path);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
    }

    // there is no derive, then default:
    if derives.is_empty() {
        derives.push(syn::parse_str("Debug").unwrap());
        derives.push(syn::parse_str("Clone").unwrap());
        derives.push(syn::parse_str("serde::Serialize").unwrap());
        derives.push(syn::parse_str("serde::Deserialize").unwrap());

        // if struct has Named fields, then check Default
        if let Fields::Named(named) = &input.fields {
            let mut all_have_default = true;
            for _ in named.named.iter() {
                // dont check type, assume all field has a Default
                // (if check, then lookup type info => more complexity)
                all_have_default &= true;
            }
            if all_have_default {
                derives.push(syn::parse_str("Default").unwrap());
            }
        }
    }

    let name = &input.ident;
    let fields = &input.fields;

    // Generate getter & setter
    let mut getters_setters: Vec<proc_macro2::TokenStream> = vec![];
    if let Fields::Named(named) = fields {
        for field in named.named.iter() {
            let fname = field.ident.as_ref().unwrap();
            let ftype = &field.ty;

            let getter_name = syn::Ident::new(&format!("{}", fname), fname.span());
            let setter_name = syn::Ident::new(&format!("set_{}", fname), fname.span());

            getters_setters.push(quote! {
                pub fn #getter_name(&self) -> &#ftype {
                    &self.#fname
                }
                pub fn #setter_name(&mut self, val: #ftype) {
                    self.#fname = val;
                }
            });
        }
    }

    let expanded = quote! {
        #[derive(#(#derives),*)]
        pub struct #name #fields

        impl #name {
            #(#getters_setters)*
        }
    };

    expanded.into()
}
