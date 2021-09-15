use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};

#[proc_macro_attribute]
pub fn subclass(_attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut subclass = syn::parse_macro_input!(item as syn::ItemEnum);
    let mut states: HashMap<Ident, LitStr> = HashMap::new();

    for variant in subclass.variants.iter_mut() {
        let attrs = &mut variant.attrs;
        // TODO: Replace with attrs.drain_filter(...)
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].path.is_ident("state") {
                let attr = attrs.remove(i);
                let code: LitStr = attr.parse_args().unwrap();
                states.insert(variant.ident.clone(), code);
            } else {
                i += 1;
            }
        }
    }

    let attributes = &subclass.attrs;
    let visibility = &subclass.vis;
    let subclass_ident = &subclass.ident;
    let variants = &subclass.variants;
    let from_str_arms = states
        .iter()
        .map(|(variant, code)| quote! { #code => Ok(Self::#variant), });
    let as_str_arms = states
        .iter()
        .map(|(variant, code)| quote! { Self::#variant => #code, });

    quote!(
        #(#attributes)*
        #visibility enum #subclass_ident {
            #variants
            Other(::std::string::String),
        }

        impl ::std::str::FromStr for #subclass_ident {
            type Err = ::std::convert::Infallible;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#from_str_arms)*
                    other => Ok(Self::Other(other.to_string())),
                }
            }
        }

        impl #subclass_ident {
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_arms)*
                    Self::Other(subclass) => subclass.as_str(),
                }
            }
        }
    )
    .into()
}
