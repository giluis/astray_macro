use super::node::{Node, NodeType};
use super::terminality::{BranchTerminality, IntoBranchTerminality};
use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::spanned::Spanned;

trait ChainQuote {
    fn chain(self, tokens: TokenStream) -> TokenStream;
}

impl ChainQuote for TokenStream {
    fn chain(mut self, tokens: TokenStream) -> TokenStream {
        self.extend(tokens);
        self
    }
}
#[derive(Debug)]
pub struct Branch {
    pub ident: syn::Ident,
    pub terminality: BranchTerminality,
    pub ty: syn::Type,
}

impl Branch {

    pub fn as_conjunct_consumption_statement(&self) -> proc_macro2::TokenStream {
            let ident = &self.ident;
            quote! {let #ident = iter}
            .chain(match &self.terminality {
                BranchTerminality::LiteralParse => quote! {
                    .parse()?;
                },
                BranchTerminality::ParseIfMatch(pattern) => quote! {
                    .parse_if_match(|input| matches!(input, #pattern))?;
                },
            })
    }

    pub fn as_disjunct_consumption_statement(&self, node_ident: &syn::Ident) -> proc_macro2::TokenStream {
            let branch_err = self.as_err_variable();
            let branch_ident = &self.ident;
            quote! {let #branch_err = iter}
            .chain(match &self.terminality {
                BranchTerminality::LiteralParse => quote! {
                    .parse()
                },
                BranchTerminality::ParseIfMatch(pattern) => quote! {
                    .parse_if_match(|input| matches!(input, #pattern))
                },
            })
            .chain(quote!{
                    .map(#node_ident::#branch_ident)
                    .hatch()?;
            })
    }

    pub fn as_err_variable(&self) -> syn::Ident {
        // the preceding underscore is necessary to avoid unused_variable warnings
        format_ident!("{}_err", &self.ident.as_snake_case())
    }
}

impl From<&syn::Field> for Branch {
    fn from(f: &syn::Field) -> Self {
        Branch {
            ident: f.ident.clone().unwrap(),
            terminality: f.as_branch_terminality(),
            ty: f.ty.clone(),
        }
    }
}

trait LeafSourceExtractable {
    fn extract_leaf_source_from_atribute(self) -> Result<syn::TypePath, syn::Error>;
}

impl From<&syn::Variant> for Branch {
    fn from(v: &syn::Variant) -> Branch {
        let ty = match &v.fields {
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => match unnamed.first() {
                Some(a) => &a.ty,
                None => unimplemented!("what to do when enum Variants are field less"),
            },
            _ => unimplemented!("Can enums have named fields"),
        };

        Branch {
            ident: v.ident.clone(),
            ty: ty.clone(),
            terminality: v.as_branch_terminality(),
        }
    }
}

trait ChangeCase {
    fn as_snake_case(&self) -> syn::Ident;
}

impl ChangeCase for syn::Ident {
    fn as_snake_case(&self) -> syn::Ident {
        syn::Ident::new(&self.to_string().to_case(Case::Snake), self.span())
    }
}
