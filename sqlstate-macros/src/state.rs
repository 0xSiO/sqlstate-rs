use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, ItemEnum, LitStr};

pub struct State {
    state_enum: ItemEnum,
    classes: HashMap<Ident, LitStr>,
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
                    classes.insert(variant.ident.clone(), code);
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

    fn generate_from_str_impl(&self) -> TokenStream {
        let class_ident = &self.state_enum.ident;

        let from_str_arms = self.classes.iter().map(|(variant, code)| {
            let parse_logic = if self.is_standard {
                // For standard types, parsing is infallible and can be unwrapped
                quote! { subclass.parse().unwrap() }
            } else {
                // For non-standard types, propagate parsing errors
                quote! { subclass.parse()? }
            };

            quote! {
                #code => Ok(Self::#variant(
                    if subclass == "000" {
                        None
                    } else {
                        Some(#parse_logic)
                    }
                )),
            }
        });

        let wildcard_arm = if self.is_standard {
            // For standard types, wrap unknown strings in 'Other' variant
            quote! { _ => Ok(Self::Other(s.to_string())), }
        } else {
            // For non-standard types, return an error
            quote! { _ => Err(crate::error::ParseError::UnknownState(s.to_string())) }
        };

        quote! {
            impl ::std::str::FromStr for #class_ident {
                type Err = crate::error::ParseError;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    // SQL standard requires length to be 5 bytes
                    if s.len() != 5 {
                        return Err(crate::error::ParseError::InvalidLength(s.len()));
                    }

                    let (class, subclass) = s.split_at(2);

                    match class {
                        #(#from_str_arms)*
                        #wildcard_arm
                    }
                }
            }
        }
    }

    fn methods_impl(&self) -> TokenStream {
        let state_ident = &self.state_enum.ident;

        let class_arms = self
            .classes
            .iter()
            .map(|(variant, code)| quote! { Self::#variant(_) => #code, });

        let class_other_arm = if self.is_standard {
            quote! { Self::Other(code) => &code[0..2], }
        } else {
            quote! {}
        };

        let subclass_arms = self.classes.keys().map(|variant| {
            quote! { Self::#variant(opt) => opt.as_ref().map(|subclass| subclass.as_str()), }
        });

        let subclass_other_arm = if self.is_standard {
            quote! { Self::Other(code) => Some(&code[2..]), }
        } else {
            quote! {}
        };

        quote! {
            impl #state_ident {
                pub fn class(&self) -> &str {
                    match self {
                        #(#class_arms)*
                        #class_other_arm
                    }
                }

                pub fn subclass(&self) -> Option<&str> {
                    match self {
                        #(#subclass_arms)*
                        #subclass_other_arm
                    }
                }
            }
        }
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all([
            self.enum_definition(),
            self.methods_impl(),
            self.generate_from_str_impl(),
        ]);
    }
}
