use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Fields, Ident, ItemEnum, LitStr};

mod class;

use self::class::Class;

#[proc_macro_attribute]
pub fn class(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let class_enum = syn::parse_macro_input!(item as ItemEnum);
    let args = syn::parse_macro_input!(attr_args as syn::AttributeArgs);

    if args.len() != 1 {
        return quote!(compile_error!("class type must be provided")).into();
    }

    let is_standard = match args[0].to_token_stream().to_string().as_str() {
        "standard" => true,
        "non_standard" => false,
        _ => {
            return quote!(compile_error!(
                "invalid argument. must be standard/non_standard"
            );)
            .into()
        }
    };

    let class = Class::new(class_enum, is_standard);
    quote!(#class).into()
}

#[proc_macro_attribute]
pub fn state(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut state_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    let args = syn::parse_macro_input!(attr_args as syn::AttributeArgs);

    let is_standard = match args[0].to_token_stream().to_string().as_str() {
        "standard" => true,
        "non_standard" => false,
        other => panic!("invalid argument '{}'", other),
    };

    // Mapping of class code -> (variant ident, whether it's a tuple variant)
    let mut classes: HashMap<LitStr, (Ident, bool)> = HashMap::new();

    for variant in state_enum.variants.iter_mut() {
        let attrs = &mut variant.attrs;
        // TODO: Replace with attrs.drain_filter(...)
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].path.is_ident("class") {
                let attr = attrs.remove(i);
                let code: LitStr = attr.parse_args().unwrap();
                classes.insert(
                    code,
                    (
                        variant.ident.clone(),
                        matches!(variant.fields, Fields::Unnamed(_)),
                    ),
                );
            } else {
                i += 1;
            }
        }
    }

    let state_ident = &state_enum.ident;

    let enum_definition = if is_standard {
        // For standard types, add a catch-all 'Other' variant
        let attributes = &state_enum.attrs;
        let visibility = &state_enum.vis;
        let variants = &state_enum.variants;
        quote! {
            #(#attributes)*
            #visibility enum #state_ident {
                #variants
                Other(::std::string::String),
            }
        }
    } else {
        // For non-standard types, leave the enum definition alone
        quote! { #state_enum }
    };

    let from_str_arms = classes.iter().map(|(code, (variant, is_tuple))| {
        if *is_tuple {
            // For standard types, parsing is infallible and can be unwrapped
            if is_standard {
                quote! { #code => Ok(Self::#variant(subclass.parse().unwrap())), }
            } else {
                quote! { #code => Ok(Self::#variant(subclass.parse()?)), }
            }
        } else {
            quote! { #code => Ok(Self::#variant), }
        }
    });

    let from_str_match = if is_standard {
        // For standard types, wrap unknown strings in 'Other' variant
        quote! {
            match class {
                #(#from_str_arms)*
                _ => Ok(Self::Other(s.to_string())),
            }
        }
    } else {
        // For non-standard types, return an error
        quote! {
            match class {
                #(#from_str_arms)*
                _ => Err(crate::error::ParseError::UnknownState(s.to_string()))
            }
        }
    };

    quote!(
        #enum_definition

        impl ::std::str::FromStr for #state_ident {
            type Err = crate::error::ParseError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                // SQL standard requires length to be 5 bytes
                if s.len() != 5 {
                    return Err(crate::error::ParseError::InvalidLength(s.len()));
                }

                let (class, subclass) = s.split_at(2);

                #from_str_match
            }
        }
    )
    .into()
}
