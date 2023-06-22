#![allow(unused_imports)]
#![feature(let_chains)]
#![feature(return_position_impl_trait_in_trait)]
// #![feature(adt_const_params)]

mod node_lib;


use node_lib::node::{Node,gen_parsable_implementation,GENERAL_LEAF_TYPE,NodeType};
use node_lib::branch::Branch;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::*;



#[proc_macro_derive(SN, attributes(pattern, stateful_leaf))]
#[allow(non_snake_case)]
pub fn SN(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let token_type_alias= syn::Ident::new(GENERAL_LEAF_TYPE, proc_macro2::Span::call_site());
    let node_impl =  gen_parsable_implementation(derive_input,token_type_alias);
    println!("{}", node_impl);
    quote!{ 
        #node_impl
    }.into()
}

#[proc_macro]
pub fn set_token(input: TokenStream) ->TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let token_type_alias= syn::Ident::new(GENERAL_LEAF_TYPE, proc_macro2::Span::call_site());
    quote!{
        type #token_type_alias = #input;
    }.into()
}

// fn repeatable(f: &syn::Field ) -> Option<syn::Ident> {
//     for attr in f.attrs.iter(){
//         if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
//             let next = attr.clone().tokens.into_iter().next();
//             if let Some(proc_macro2::TokenTree::Group(g)) = next{
//                 let mut giter = g.stream().into_iter();
//                 let _each = giter.next();
//                 let _equalsign = giter.next();
//                 let arg = match giter.next().unwrap(){
//                     proc_macro2::TokenTree::Literal(l) => l,
//                     tt => panic!("Expected string, found {}", tt),
//                 };
//                 match syn::Lit::new(arg) {
//                     syn::Lit::Str(s) => {
//                         return Some(syn::Ident::new( &s.value(), s.span() ));
//                     },
//                     lit => panic!("Expected string, found {:?}", lit),
//                 };
//
//             }
//         }
//     };
//     return None;
// }
//

