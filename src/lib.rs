#![feature(return_position_impl_trait_in_trait)]

mod node;

use node::{build_parse_fn,TOKEN_TYPE_ALIAS_STR};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::*;



#[proc_macro_derive(SN, attributes(pattern, stateful_leaf))]
#[allow(non_snake_case)]
pub fn SN(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let node_impl =  build_parse_fn(derive_input);
    node_impl.into()
}

#[proc_macro]
pub fn set_token(input: TokenStream) ->TokenStream {
    let token_type_alias = syn::Ident::new(TOKEN_TYPE_ALIAS_STR, proc_macro2::Span::call_site());
    let input: proc_macro2::TokenStream = input.into();

    quote!{
        type #token_type_alias = #input;
    }.into()
}
