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

        Class {
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

        let from_str_match = if self.is_standard {
            quote! {
                match s {
                    #(#from_str_arms)*
                    other => Ok(Self::Other(other.to_string())),
                }
            }
        } else {
            quote! {
                match s {
                    #(#from_str_arms)*
                    other => Err(crate::error::ParseError::UnknownSubclass(other.to_string())),
                }
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

    fn as_str_impl(&self) -> TokenStream {
        let class_ident = &self.class_enum.ident;

        let as_str_arms = self
            .subclasses
            .iter()
            .map(|(variant, code)| quote! { Self::#variant => #code, });

        let as_str_match = if self.is_standard {
            quote! {
                match self {
                    #(#as_str_arms)*
                    Self::Other(subclass) => subclass.as_str(),
                }
            }
        } else {
            quote! {
                match self {
                    #(#as_str_arms)*
                }
            }
        };

        quote! {
            impl #class_ident {
                pub fn as_str(&self) -> &str {
                    #as_str_match
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
