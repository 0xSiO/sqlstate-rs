use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, ItemEnum, LitStr};

pub struct Class {
    class_enum: ItemEnum,
    subclasses: HashMap<Ident, LitStr>,
    is_standard: bool,
}

impl Class {
    pub fn new(mut class_enum: ItemEnum, is_standard: bool) -> Self {
        let mut subclasses = HashMap::new();

        for variant in class_enum.variants.iter_mut() {
            let attrs = &mut variant.attrs;
            // TODO: Replace with attrs.drain_filter(...)
            let mut i = 0;
            while i < attrs.len() {
                if attrs[i].path.is_ident("subclass") {
                    let attr = attrs.remove(i);
                    let code: LitStr = attr.parse_args().unwrap();
                    subclasses.insert(variant.ident.clone(), code);
                } else {
                    i += 1;
                }
            }
        }

        Self {
            class_enum,
            subclasses,
            is_standard,
        }
    }

    fn enum_definition(&self) -> TokenStream {
        if self.is_standard {
            let attributes = &self.class_enum.attrs;
            let visibility = &self.class_enum.vis;
            let class_ident = &self.class_enum.ident;
            let variants = &self.class_enum.variants;
            quote! {
                #(#attributes)*
                #visibility enum #class_ident {
                    #variants
                    Other(::std::string::String),
                }
            }
        } else {
            let enum_definition = &self.class_enum;
            quote! {
                #enum_definition
            }
        }
    }

    fn from_str_impl(&self) -> TokenStream {
        let class_ident = &self.class_enum.ident;

        let err_type = if self.is_standard {
            quote! { ::std::convert::Infallible }
        } else {
            quote! { crate::error::ParseError }
        };

        let from_str_arms = self
            .subclasses
            .iter()
            .map(|(variant, code)| quote! { #code => Ok(Self::#variant), });

        let wildcard_arm = if self.is_standard {
            // For standard types, wrap unknown strings in 'Other' variant
            quote! { _ => Ok(Self::Other(s.to_string())), }
        } else {
            // For non-standard types, return an error
            quote! { _ => Err(crate::error::ParseError::UnknownSubclass(s.to_string())) }
        };

        quote! {
            impl ::std::str::FromStr for #class_ident {
                type Err = #err_type;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    match s {
                        #(#from_str_arms)*
                        #wildcard_arm
                    }
                }
            }
        }
    }

    fn as_str_impl(&self) -> TokenStream {
        let class_ident = &self.class_enum.ident;

        let as_str_arms = self
            .subclasses
            .iter()
            .map(|(variant, code)| quote! { Self::#variant => #code, });

        let other_arm = if self.is_standard {
            quote! { Self::Other(subclass) => subclass.as_str(), }
        } else {
            quote!()
        };

        quote! {
            impl #class_ident {
                pub fn as_str(&self) -> &str {
                    match self {
                        #(#as_str_arms)*
                        #other_arm
                    }
                }
            }
        }
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all([
            self.enum_definition(),
            self.from_str_impl(),
            self.as_str_impl(),
        ]);
    }
}
