use super::branch::Branch;
use proc_macro2::TokenStream;
use quote::*;
use std::iter::Peekable;
use std::slice::Iter;
use syn::DataStruct;
use syn::DeriveInput;
use syn::TypePath;

pub const GENERAL_LEAF_TYPE: &str = "AstrayToken";

#[derive(Debug)]
pub enum NodeType {
    ProductNode,
    SumNode,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub ident: syn::Ident,
    branches: Vec<Branch>,
}

// /**
//  *  impl Parsable<#Token> for #Type {
//  *      fn parse(iter: &mut TokenIter) -> Result<#Type, ParseError<#Token>> {
//  *          (
//  *          // in case it is a struct Node
//  *          let #field_name = iter.parse().map_err(|err| ParseError::from_conjunct_error(err))?;
//  *          // in case it is an enum Node
//  *          let #field_name ## _err = iter.parse()?.map(|result: #field_type |#Type::#field_name(result)).hatch()?;
//  *          ) * // repeat for each field
//  *          
//  *          // if struct Node
//  *          Ok(#Type {#(#field_name)*})
//  *          // else if enum Node
//  *          Err(ParseError::from_disjunct_errors(#(#field_name##_err)*))
//  *      }
//  * }
// */

pub fn gen_parsable_implementation(
    derive_input: DeriveInput,
    token_type_alias: syn::Ident,
) -> TokenStream {
    let node = Node::from_derive_input(derive_input);
    let branch_consumption = node.as_consumption_statements();
    let node_construction = node.as_construction_statement();
    let node_ident = node.ident;
    quote! {
        impl Parsable<#token_type_alias> for #node_ident {
            fn parse(iter: &mut TokenIter<#token_type_alias>) -> Result<#node_ident, ParseError<#token_type_alias>> {
                #(#branch_consumption)*
                #node_construction
            }
        }
    }
}

impl Node {
    pub fn from_derive_input(derive_input: DeriveInput) -> Self {
        let (branches, node_type) = match derive_input.data {
            syn::Data::Struct(data_struct) => {
                (data_struct.fields.into_branches(), NodeType::ProductNode)
            }
            syn::Data::Enum(data_enum) => (data_enum.variants.into_branches(), NodeType::SumNode),
            _ => unimplemented!("Nodes from unions are not implemented"),
        };
        Node {
            branches,
            node_type,
            ident: derive_input.ident,
        }
    }

    pub fn as_consumption_statements(&self) -> Vec<TokenStream> {
        self.branches
            .iter()
            .map(|b| match self.node_type {
                NodeType::ProductNode => b.as_conjunct_consumption_statement(),
                NodeType::SumNode => b.as_disjunct_consumption_statement(&self.ident),
            })
            .collect()
    }

    fn as_construction_statement(&self) -> TokenStream {
        match self.node_type {
            NodeType::ProductNode => self.as_conjunct_construction_statement(),
            NodeType::SumNode => self.as_disjunct_construction_statement(),
        }
    }

    fn as_conjunct_construction_statement(&self) -> TokenStream {
        let branches = self.branches.iter().map(|b| &b.ident);
        let ident = &self.ident;
        quote! {Ok(#ident{#(#branches,)*})}
    }

    fn as_disjunct_construction_statement(&self) -> TokenStream {
        let branches = self.branches.iter().map(|b| b.as_err_variable());
        let ty = &self.ident;
        quote! {Err(ParseError::from_disjunct_errors::<#ty>(iter.current, vec![#(#branches,)*]))}
    }
}

trait IntoBranches {
    fn into_branches(self) -> Vec<Branch>;
}

impl IntoBranches for syn::Fields {
    fn into_branches<'a>(self) -> Vec<Branch> {
        match self {
            syn::Fields::Named(syn::FieldsNamed {
                named: fields_named,
                ..
            }) => fields_named
                .pairs()
                .map(|f| f.into_value().into())
                .collect(),
            _ => unimplemented!(
                "Unimplemented: Unnamed syn::fields. Should this be allowed by the API?"
            ),
        }
    }
}

impl IntoBranches for syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> {
    fn into_branches<'a>(self) -> Vec<Branch> {
        self.iter().map(|v| v.into()).collect()
    }
}
