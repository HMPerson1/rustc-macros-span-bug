extern crate proc_macro;

#[proc_macro_attribute]
pub fn my_proc_macro(
    _attr: proc_macro::TokenStream, item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item.into_iter().collect()
}
