use darling::{ast, util, FromDeriveInput, FromField, FromMeta, FromVariant};

#[derive(FromDeriveInput)]
#[darling(supports(enum_tuple), attributes(enum_ptr), forward_attrs(repr))]
pub struct Input {
    pub ident: syn::Ident,
    pub vis: syn::Visibility,
    pub generics: syn::Generics,
    pub attrs: Vec<syn::Attribute>,
    pub data: ast::Data<Variant, ()>,

    pub copy: util::Flag,
    pub borrow: Option<util::Override<BorrowConf>>,
    pub borrow_mut: Option<util::Override<BorrowConf>>,
}

#[derive(FromVariant)]
#[darling(attributes(enum_ptr))]
pub struct Variant {
    pub ident: syn::Ident,
    pub discriminant: Option<syn::Expr>,
    pub fields: ast::Fields<Field>,

    pub skip: util::Flag,
    pub skip_borrow: util::Flag,
    pub skip_borrow_mut: util::Flag,
}

#[derive(FromField)]
pub struct Field {
    pub ty: syn::Type,
}

#[derive(FromMeta, Default, Clone)]
pub struct BorrowConf {
    pub name: Option<String>,
}
