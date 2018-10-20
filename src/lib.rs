extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn reforest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item.into_iter().collect()
}
