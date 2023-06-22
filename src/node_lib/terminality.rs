use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug)]
pub enum BranchTerminality {
    LiteralParse,
    ParseIfMatch(syn::Pat),
}


pub trait HasType {
    fn get_type(&self) -> syn::Type;
}

//TODO: how to remove this clone
impl HasType for &syn::Field {
    fn get_type(&self) -> syn::Type {
        self.ty.clone()
    }
}

//TODO: how to remove this clone
impl HasType for &syn::Variant {
    fn get_type(&self) -> syn::Type {
        match &self.fields {
            syn::Fields::Named(syn::FieldsNamed { .. }) => {
                unimplemented!("Enums with inline structs as types have not been implemented yet")
            }
            syn::Fields::Unnamed(unamed_fields) => {
                unamed_fields.unnamed.first().unwrap().ty.clone()
            }
            _ => unimplemented!("Unit variants (variants without types) have not been implemented"),
        }
    }
}

pub trait HasAttributes
{
    fn get_attrs(&self) -> impl Iterator<Item = &syn::Attribute>;
}

impl<'a> HasAttributes for &'a syn::Field {
    fn get_attrs(&self) -> impl Iterator<Item = &syn::Attribute> {
        self.attrs.iter()
    }
}

impl<'a> HasAttributes for &'a syn::Variant {
    fn get_attrs(&self) -> impl Iterator<Item = &syn::Attribute> {
        self.attrs.iter()
    }
}
