use proc_macro::{TokenStream};
mod macros;
mod adv_macros;

#[proc_macro_attribute]
pub fn record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    adv_macros::struct_data_adv::create_record(_attr, item)
}