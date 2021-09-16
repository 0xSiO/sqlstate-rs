use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Fields, Ident, ItemEnum, LitStr};

pub struct State {
    state_enum: ItemEnum,
    classes: HashMap<Ident, (LitStr, bool)>,
    is_standard: bool,
}

impl State {
    pub fn new(mut state_enum: ItemEnum, is_standard: bool) -> Self {
        let mut classes = HashMap::new();

        for variant in state_enum.variants.iter_mut() {
            let attrs = &mut variant.attrs;
            // TODO: Replace with attrs.drain_filter(...)
            let mut i = 0;
            while i < attrs.len() {
                if attrs[i].path.is_ident("class") {
                    let attr = attrs.remove(i);
                    let code: LitStr = attr.parse_args().unwrap();
                    classes.insert(
                        variant.ident.clone(),
                        (code, matches!(variant.fields, Fields::Unnamed(_))),
                    );
                } else {
                    i += 1;
                }
            }
        }

        Self {
            state_enum,
            classes,
            is_standard,
        }
    }

    fn enum_definition(&self) -> TokenStream {
        if self.is_standard {
            let attributes = &self.state_enum.attrs;
            let visibility = &self.state_enum.vis;
            let class_ident = &self.state_enum.ident;
            let variants = &self.state_enum.variants;
            quote! {
                #(#attributes)*
                #visibility enum #class_ident {
                    #variants
                    Other(::std::string::String),
                }
            }
        } else {
            let enum_definition = &self.state_enum;
            quote! {
                #enum_definition
            }
        }
    }

    fn from_str_impl(&self) -> TokenStream {
        let class_ident = &self.state_enum.ident;

        let err_type = if self.is_standard {
            quote! { ::std::convert::Infallible }
        } else {
            quote! { crate::error::ParseError }
        };

        let from_str_arms = self.classes.iter().map(|(code, (variant, is_tuple))| {
            if *is_tuple {
                if self.is_standard {
                    // For standard types, parsing is infallible and can be unwrapped
                    quote! { #code => Ok(Self::#variant(subclass.parse().unwrap())), }
                } else {
                    // For non-standard types, propagate parsing errors
                    quote! { #code => Ok(Self::#variant(subclass.parse()?)), }
                }
            } else {
                quote! { #code => Ok(Self::#variant), }
            }
        });

        let wildcard_arm = if self.is_standard {
            // For standard types, wrap unknown strings in 'Other' variant
            quote! { _ => Ok(Self::Other(s.to_string())), }
        } else {
            // For non-standard types, return an error
            quote! { _ => Err(crate::error::ParseError::UnknownState(s.to_string())) }
        };

        let from_str_match = quote! {
            match class {
                #(#from_str_arms)*
                #wildcard_arm
            }
        };

        quote! {
            impl ::std::str::FromStr for #class_ident {
                type Err = #err_type;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    #from_str_match
                }
            }
        }
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all([self.enum_definition(), self.from_str_impl()]);
    }
}
