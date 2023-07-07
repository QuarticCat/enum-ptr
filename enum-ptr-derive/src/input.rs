use darling::{ast, util, FromDeriveInput, FromField, FromMeta, FromVariant};

#[derive(FromDeriveInput)]
#[darling(supports(enum_tuple), attributes(enum_ptr), forward_attrs(repr))]
pub struct Input {
    pub ident: syn::Ident,
    // pub vis: syn::Visibility,
    pub generics: syn::Generics,
    pub attrs: Vec<syn::Attribute>,
    pub data: ast::Data<Variant, ()>,

    pub copy: util::Flag,
    pub borrow: util::Flag,
    pub borrow_mut: util::Flag,
}

#[derive(FromVariant)]
pub struct Variant {
    pub ident: syn::Ident,
    pub discriminant: Option<syn::Expr>,
    pub fields: ast::Fields<Field>,

    pub skip_list: Option<util::Override<SkipList>>,
}

#[derive(FromField)]
pub struct Field {
    // pub vis: syn::Visibility,
    pub ty: syn::Type,
}

#[derive(FromMeta)]
pub struct SkipList {
    pub borrow: util::Flag,
    pub borrow_mut: util::Flag,
}
