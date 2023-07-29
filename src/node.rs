use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::*;
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Field, Fields, Variant};

pub const TOKEN_TYPE_ALIAS_STR: &str = "AstrayToken";
fn get_token_type_alias() -> syn::Ident {
    syn::Ident::new(TOKEN_TYPE_ALIAS_STR, proc_macro2::Span::call_site())
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
    let token_type_alias = get_token_type_alias();
    let err_variable = variant.ident.as_error_variable();
    let variant_name = &variant.ident;
    let mut parse_if_match_type_specification = quote! {_};
    let variant_construction = match &variant.fields {
        //TODO: Add support for multiple unnamed fields in an enum
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => match unnamed.first() {
            // TODO: find a better name for this
            Some(_unnamed_for_now) => quote! {
                .map(#node_ident::#variant_name)
            },
            // TODO: Check whether or not this is upheld by syn
            None => {
                unreachable!("If 'unnamed' is empty, then the field should be Unit, not Unnamed.")
            }
        },
        syn::Fields::Named(_) => {
            unimplemented!("Named fields in enums are not yet supported")
        }
        syn::Fields::Unit => {
            parse_if_match_type_specification =
                quote! {<#token_type_alias as Parsable<#token_type_alias>>::ApplyMatchTo};
            quote! {
                .if_ok(#node_ident::#variant_name)
            }
        }
    };
    let parse_statement = as_branch_terminality(variant, parse_if_match_type_specification);
    quote! {let #err_variable = #parse_statement}
        .chain(variant_construction)
        .chain(quote! {
            .hatch()?;
        })
}

fn conjunct(field: &Field) -> TokenStream {
    let field_name = &field.ident;
    let parse_statement = as_branch_terminality(field, quote! {_});
    quote! {let #field_name = #parse_statement}.chain(quote! {?;})
}

fn conjunct_all(fields: Fields, node_ident: &syn::Ident) -> TokenStream {
    let (consumption_statements, idents) = match fields {
        syn::Fields::Named(syn::FieldsNamed { ref named, .. }) => {
            (
                named.pairs().map(|f| conjunct(f.into_value())),
                named.pairs().map(|f| match f.into_value().ident {
                    Some(ref a) => a,
                    // TODO: improve this error message
                    None => unimplemented!(
                        "Ident will always exist here, since named fields always have idents"
                    ),
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

pub fn build_parse_fn(derive_input: DeriveInput) -> TokenStream {
    let token_type_alias = get_token_type_alias();
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

// TODO: find better name for this function and concept
fn as_branch_terminality<T>(ty: &T, type_specification: TokenStream) -> TokenStream
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
            let pat_string = quote!{#pat}.to_string();
            quote!(iter.parse_if_match::<_,#type_specification>(|node| matches!(node, #pat),Some(#pat_string))) 
        }
    }
}

/* --- Useful trait implementations --- */

pub trait ToErrVariable {
    fn as_error_variable(&self) -> syn::Ident;
}

impl ToErrVariable for syn::Ident {
    fn as_error_variable(&self) -> syn::Ident {
        let mut err_variable = self.to_string().to_case(Case::Snake);
        err_variable.push_str("_err");
        syn::Ident::new(err_variable.as_str(), self.span())
    }
}

pub trait ChainQuote {
    fn chain(self, tokens: TokenStream) -> TokenStream;
}

impl ChainQuote for TokenStream {
    fn chain(mut self, tokens: TokenStream) -> TokenStream {
        self.extend(tokens);
        self
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
