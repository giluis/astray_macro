use super::branch::Branch;
use convert_case::Case;
use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::*;
use std::iter::Map;
use std::iter::Peekable;
use std::slice::Iter;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::TypePath;
use syn::Variant;

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
trait ToErrVariable {
    fn as_error_variable(&self) -> syn::Ident;
}

impl ToErrVariable for syn::Ident {
    fn as_error_variable(&self) -> syn::Ident {
        let mut err_variable = self.to_string().to_case(Case::Snake);
        err_variable.push_str("_err");
        syn::Ident::new(err_variable.as_str(), self.span())
    }
}

trait ChainQuote {
    fn chain(self, tokens: TokenStream) -> TokenStream;
}

impl ChainQuote for TokenStream {
    fn chain(mut self, tokens: TokenStream) -> TokenStream {
        self.extend(tokens);
        self
    }
}

trait IfOk<T, E> {
    fn if_ok(self, value: T) -> Result<T, E>;
}

impl<P, T, E> IfOk<T, E> for Result<P, E> {
    fn if_ok(self, value: T) -> Result<T, E> {
        match self {
            Ok(_) => Ok(value),
            Err(err) => Err(err),
        }
    }
}

fn disjunct_all(variants: Punctuated<Variant, Comma>, node_ident: &syn::Ident) -> TokenStream {
    let consumption_statements = variants.iter().map(|v| disjunct(v, node_ident));
    let err_vars = variants.iter().map(|v| v.ident.as_error_variable());
    quote! {
        #(#consumption_statements)*
        Err(ParseError::from_disjunct_errors::<#node_ident>(iter.current, vec![#(#err_vars,)*]))
    }
}

fn disjunct(variant: &Variant, node_ident: &syn::Ident) -> TokenStream {
    let err_variable = variant.ident.as_error_variable();
    let variant_name = &variant.ident;
    let parse_statement = as_branch_terminality(variant);
    quote! {let #err_variable = #parse_statement}
        .chain(match &variant.fields {
            //TODO: Add support for multiple unnamed fields in an enum
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => match unnamed.first() {
                // TODO: find a better name for this
                Some(_unnamed_for_now) => quote! {
                    .map(#node_ident::#variant_name)
                },
                // TODO: Check whether or not this is upheld by syn
                None => unreachable!(
                    "If 'unnamed' is empty, then the field should be Unit, not Unnamed."
                ),
            },
            syn::Fields::Named(_) => {
                unimplemented!("Named fields in enums are not yet supported")
            }
            syn::Fields::Unit => quote! {
                .if_ok(#node_ident::#variant_name)
            },
        })
        .chain(quote! {
            .hatch()?;
        })
}

fn conjunct(field: &Field) -> TokenStream {
    let field_name = &field.ident;
    let parse_statement = as_branch_terminality(field);
    quote! {let #field_name = #parse_statement}.chain(quote! {
        ?;
    })
}

fn conjunct_all(fields: Fields, node_ident: &syn::Ident) -> TokenStream {
    let (consumption_statements, idents) = match fields {
        syn::Fields::Named(syn::FieldsNamed { ref named, .. }) => {
            (
                named.pairs().map(|f| conjunct(f.into_value())),
                named.pairs().map(|f| match f.into_value().ident {
                    Some(ref a) => a,
                    // TODO: improve this error message
                    None => unimplemented!("Ident will always exist here, since named fields always have idents"),
                }),
            )
        }
        Fields::Unnamed(_) => todo!("Tuple structs are not yet supported"),
        // TODO: Comprehensive error message with correct SPan
        Fields::Unit => unimplemented!("Unit structs can't derive SN"),
    };

    quote! {#(#consumption_statements)*
         Ok(#node_ident {#(#idents),*})}
}

pub fn gen_parsable_implementation(
    derive_input: DeriveInput,
    token_type_alias: syn::Ident,
) -> TokenStream {
    
    let parse_fn = match derive_input.data {
        syn::Data::Struct(data_struct) => conjunct_all(data_struct.fields, &derive_input.ident),
        syn::Data::Enum(data_enum) => disjunct_all(data_enum.variants, &derive_input.ident),
        _ => unimplemented!("Nodes from unions are not implemented"),
    };
    let node_ident = derive_input.ident;
    quote! {
        impl Parsable<#token_type_alias> for #node_ident {
            fn parse(iter: &mut TokenIter<#token_type_alias>) -> Result<#node_ident, ParseError<#token_type_alias>> {
                #parse_fn
            }
        }
    }
}


impl Node {
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
            syn::Fields::Unnamed(_) => todo!("Tuple structs are not implemented yet"),
            // TODO: Add check here to make sure error span is correct and message is properly shown
            syn::Fields::Unit => unimplemented!("Unit structs cannot derive SN"),
        }
    }
}

impl IntoBranches for syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> {
    fn into_branches<'a>(self) -> Vec<Branch> {
        self.iter().map(|v| v.into()).collect()
    }
}

fn as_branch_terminality<T>(ty: &T) -> TokenStream
where
    T: HasAttributes,
{
    match ty.get_attrs().find(
        |attr| /* attr.path.segments.len() == 1 && */ attr.path.segments[0].ident == "pattern",
    ) {
        None => quote! {iter.parse()},
        Some(attr) => {
            let pat = attr
                .parse_args::<syn::Pat>()
                // TODO: Make sure error span is correct and type, field and incorrect pattern are mentioned in message
                .expect("Incorrect pattern was provided");
            quote!(iter.parse_if_match(|node| matches!(node, #pat)))
        }
    }
}

pub trait HasAttributes {
    fn get_attrs(&self) -> impl Iterator<Item = &syn::Attribute>;
}

impl HasAttributes for syn::Field {
    fn get_attrs(&self) -> impl Iterator<Item = &syn::Attribute> {
        self.attrs.iter()
    }
}

impl HasAttributes for syn::Variant {
    fn get_attrs(&self) -> impl Iterator<Item = &syn::Attribute> {
        self.attrs.iter()
    }
}
