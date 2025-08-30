use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Fields, ItemStruct, Lit, Meta, NestedMeta, Path};

pub(crate) fn create_record(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);

    let mut derives: Vec<Path> = Vec::new();
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
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
    }

    // Nếu user không truyền derive thì thêm mặc định
    if derives.is_empty() {
        derives.push(syn::parse_str("Debug").unwrap());
        derives.push(syn::parse_str("Clone").unwrap());
        derives.push(syn::parse_str("serde::Serialize").unwrap());
        derives.push(syn::parse_str("serde::Deserialize").unwrap());

        // Nếu struct có Named fields thì kiểm tra Default
        if let Fields::Named(named) = &input.fields {
            let mut all_have_default = true;
            for _ in named.named.iter() {
                // Không check type phức tạp, chỉ assume tất cả field có Default
                // (muốn check thật sự thì phải lookup type info => phức tạp hơn)
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
    let mut getters_setters = Vec::new();
    if let Fields::Named(named) = fields {
        for field in named.named.iter() {
            let fname = field.ident.as_ref().unwrap();
            let ftype = &field.ty;

            let getter_name = syn::Ident::new(&format!("get_{}", fname), fname.span());
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
