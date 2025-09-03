use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Fields, ItemStruct, Lit, Path, Token};

// type AttributeArgs = Punctuated<Expr, Token![,]>;
pub(crate) fn create_record(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Expr, Token![,]>::parse_terminated);
    let input = parse_macro_input!(item as ItemStruct);

    let mut derives: Vec<Path> = vec![];
    for arg in args {
        match arg {
            Expr::Path(expr_path) => {
                // ex: #[record(SomeAttr)]
                // println!("Found Path: {}", path.into_token_stream());
                derives.push(expr_path.path);
            }
            Expr::Assign(assign) => {
                // ex: #[record(extra = "yes")]
                // let ident = nv.path.into_token_stream().to_string();
                // let lit = nv.lit.into_token_stream().to_string();
                // println!("Found NameValue: {} = {}", ident, lit);
                if let Expr::Path(left) = *assign.left {
                    if left.path.is_ident("derive") {
                        if let Expr::Lit(right) = *assign.right {
                            if let Lit::Str(litstr) = right.lit {
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
                }
            }
            Expr::Call(call) => {
                // ex: #[record(derive(Debug, Clone))]
                if let Expr::Path(expr_path) = *call.func {
                    if expr_path.path.is_ident("derive") {
                        for arg in call.args {
                            if let Expr::Path(path) = arg {
                                // println!("  List item Path: {}", path.clone().into_token_stream());
                                derives.push(path.path);
                            }
                        }
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

    // Generate associated func getter & setter
    let mut getters_setters: Vec<proc_macro2::TokenStream> = vec![];
    let mut assoc_fn = quote! {};
    if let Fields::Named(named) = fields {
        let mut assoc_fn_args: Vec<proc_macro2::TokenStream> = vec![];
        let mut assoc_fn_params: Vec<proc_macro2::TokenStream> = vec![];
        for field in named.named.iter() {
            let fname = field.ident.as_ref().unwrap();
            let ftype = &field.ty;

            assoc_fn_args.push(quote! { #fname: #ftype });
            assoc_fn_params.push(quote! { #fname });

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
        // check impl
        assoc_fn = quote! {
            pub fn new( #(#assoc_fn_args),* ) -> Self {
                Self {
                    #(#assoc_fn_params),*
                }
            }
        }
    }

    let expanded = quote! {
        #[derive(#(#derives),*)]
        pub struct #name #fields

        impl #name {
            #assoc_fn
            #(#getters_setters)*
        }
    };

    TokenStream::from(expanded)
}
