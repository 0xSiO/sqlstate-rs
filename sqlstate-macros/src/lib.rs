use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemEnum};

mod class;
mod state;

use crate::state::State;

use self::class::Class;

fn is_standard(args: AttributeArgs) -> bool {
    if args.len() != 1 {
        panic!("type must be provided (standard/non_standard)");
    }

    match args[0].to_token_stream().to_string().as_str() {
        "standard" => true,
        "non_standard" => false,
        other => panic!("invalid argument '{}'", other),
    }
}

#[proc_macro_attribute]
pub fn class(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let class_enum = syn::parse_macro_input!(item as ItemEnum);
    let args = syn::parse_macro_input!(attr_args as AttributeArgs);

    let class = Class::new(class_enum, is_standard(args));
    quote!(#class).into()
}

#[proc_macro_attribute]
pub fn state(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let state_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    let args = syn::parse_macro_input!(attr_args as AttributeArgs);

    let state = State::new(state_enum, is_standard(args));
    quote!(#state).into()
}
