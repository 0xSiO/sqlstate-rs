use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};

#[proc_macro_attribute]
pub fn subclass(_attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut subclass = syn::parse_macro_input!(item as syn::ItemEnum);
    let _states: HashMap<Ident, LitStr> = HashMap::new();

    for variant in subclass.variants.iter_mut() {
        let attrs = &mut variant.attrs;
        // TODO: Replace with attrs.drain_filter(...)
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].path.is_ident("state") {
                attrs.remove(i);
            } else {
                i += 1;
            }
        }
    }

    quote!(#subclass).into()
}
