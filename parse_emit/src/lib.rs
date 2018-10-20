extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn my_proc_macro(
    _attr: proc_macro::TokenStream, item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse::<syn::Item>(item).unwrap();
    let tts: TokenStream = quote! {
        #item
    }.into();
    tts.into()
}
