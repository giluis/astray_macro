use quote::*;

#[derive(Debug)]
pub enum BranchTerminality {
    Reference(syn::Type),
    StatelessLeaf { source: syn::Expr },
}

impl BranchTerminality {
    pub fn as_conjunct_fn_call(&self, ty: &syn::Type) -> proc_macro2::TokenStream {
        match self {
            Self::StatelessLeaf { source } => {
                let pattern_string = quote! {#source}.to_string();
                // TODO: remove uncecessary to string: string can be stored in binary
                quote! { .expect_msg(|x|matches!(x,#source), #pattern_string.to_string()) }},
            Self::Reference(_ty) => quote! { .parse::<#ty>() },
        }
    }

    pub fn as_disjunct_fn_call(
        &self,
        node_name: &syn::Ident,
        ty: &syn::Type,
        branch_ident: &syn::Ident,
    ) -> proc_macro2::TokenStream {
        match self {
            Self::Reference(_inner_ty) => quote! { .parse::<#ty>()
            .map(|v| #node_name::#branch_ident(v))
            .hatch() },
            Self::StatelessLeaf { source } => {
                let pattern_string = quote! {#source}.to_string();
                // TODO: remove uncecessary to string: string can be stored in binary
                quote! { .expect_msg(|x|matches!(x,#source),#pattern_string.to_string())
                .map(|v| #node_name::#branch_ident(v))
                .hatch() }
            }
        }
    }
}

pub trait IntoBranchTerminality {
    fn as_branch_terminality<'a>(&'a self) -> BranchTerminality
    where
        Self: HasAttributes<'a> + syn::spanned::Spanned + Sized + HasType,
    {
        match self.get_attrs().find(
            |attr| /* attr.path.segments.len() == 1 && */ attr.path.segments[0].ident == "from",
        ) {
            None => BranchTerminality::Reference(self.get_type()),
            Some(attr) => {
                let source = attr
                    .parse_args::<syn::Expr>()
                    .expect("Could not extract leaf source from attribute");
                BranchTerminality::StatelessLeaf { source }
            }
        }
    }
}

impl<'a> IntoBranchTerminality for &'a syn::Variant {}
impl<'a> IntoBranchTerminality for &'a syn::Field {}

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

pub trait HasAttributes<'a> {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute>;
}

impl<'a> HasAttributes<'a> for &'a syn::Field {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute> {
        self.attrs.iter()
    }
}

impl<'a> HasAttributes<'a> for &'a syn::Variant {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute> {
        self.attrs.iter()
    }
}
