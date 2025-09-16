use proc_macro::TokenStream;
mod adv_macros;
mod macros;
#[proc_macro_attribute]
pub fn record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    adv_macros::util_struct::create_record(_attr, item)
}

#[proc_macro]
pub fn sum(item: TokenStream) -> TokenStream {
    adv_macros::util_func::calculate_sum(item)
}

#[proc_macro_attribute]
pub fn with(attr: TokenStream, item: TokenStream) -> TokenStream {
    adv_macros::util_func::create_with(attr, item)
}

#[proc_macro_derive(Crud, attributes(crud, column))]
pub fn derive_crud(input: TokenStream) -> TokenStream {
    adv_macros::util_func::create_crud(input)
}
