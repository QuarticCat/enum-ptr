use darling::ast;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

use crate::{BorrowConf, Input};

pub fn gen_basic(input: &Input) -> TokenStream {
    let input_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let original_type = quote!(#input_ident #ty_generics);
    let compact_type = quote!(::enum_ptr::Compact<#original_type>);

    let ast::Data::Enum(variants) = &input.data else { unreachable!() };
    let min_align = variants.len().next_power_of_two();
    let tag_mask = min_align - 1;
    let mut asserts = Vec::new();
    for variant in variants {
        let variant_ident = &variant.ident;
        let field_type = &variant.fields.iter().next().unwrap().ty;
        let assert_msg = format!("`{input_ident}::{variant_ident}` has no enough alignment");
        // TODO: change to static asserts when available
        asserts.push(quote! {
            assert!(
                <#field_type as ::enum_ptr::Aligned>::ALIGNMENT >= #min_align,
                #assert_msg
            );
        });
    }

    quote! {
        impl #impl_generics From<#original_type> for #compact_type #where_clause {
            #[inline]
            fn from(value: #original_type) -> Self {
                #(#asserts)*
                unsafe { ::core::mem::transmute(::enum_ptr::compact(value)) }
            }
        }

        impl #impl_generics From<#compact_type> for #original_type #where_clause {
            #[inline]
            fn from(value: #compact_type) -> Self {
                unsafe { ::enum_ptr::extract(::core::mem::transmute(value), #tag_mask) }
            }
        }
    }
    .into()
}

pub fn gen_copy(input: &Input) -> TokenStream {
    let input_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let original_type = quote!(#input_ident #ty_generics);
    let compact_type = quote!(::enum_ptr::CompactCopy<#original_type>);

    quote! {
        impl #impl_generics From<#original_type> for #compact_type #where_clause {
            #[inline]
            fn from(value: #original_type) -> Self {
                ::enum_ptr::Compact::from(value).into()
            }
        }

        impl #impl_generics From<#compact_type> for #original_type #where_clause {
            #[inline]
            fn from(value: #compact_type) -> Self {
                ::enum_ptr::Compact::from(value).into()
            }
        }
    }
    .into()
}

pub fn gen_borrow(input: &Input, conf: &BorrowConf) -> TokenStream {
    let input_ident = &input.ident;
    let input_vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let original_type = quote!(#input_ident #ty_generics);

    let mut ref_generics = input.generics.clone();
    ref_generics.params.insert(0, parse_quote!('enum_ptr));
    match &mut ref_generics.where_clause {
        Some(w) => w.predicates.push(parse_quote!(#original_type: 'enum_ptr)),
        none => *none = parse_quote!(where #original_type: 'enum_ptr),
    }
    let (ref_impl_generics, ref_ty_generics, ref_where_clause) = ref_generics.split_for_impl();
    let ref_ident = match &conf.name {
        Some(name) => format_ident!("{name}"),
        None => format_ident!("{input_ident}Ref"),
    };
    let ref_derive = conf.derive.as_ref().map(|d| quote!(#[#d]));

    let ast::Data::Enum(variants) = &input.data else { unreachable!() };
    let mut ref_variants = Vec::new();
    let mut match_arms = Vec::new();
    for variant in variants {
        let variant_ident = &variant.ident;
        let field_type = &variant.fields.iter().next().unwrap().ty;
        let skip = variant.skip.is_present() || variant.skip_borrow.is_present();
        if !skip {
            ref_variants.push(quote! {
                #variant_ident(<#field_type as ::enum_ptr::FieldDeref>::Target<'enum_ptr>),
            });
            match_arms.push(quote! {
                Self::#variant_ident(inner) => Self::Target::#variant_ident(
                    ::enum_ptr::FieldDeref::force_deref(inner)
                ),
            });
        } else {
            ref_variants.push(quote! {
                #variant_ident(::core::marker::PhantomData<*const #field_type>),
            });
            match_arms.push(quote! {
                Self::#variant_ident(_) => Self::Target::#variant_ident(
                    ::core::marker::PhantomData
                ),
            });
        }
    }

    quote! {
        #ref_derive
        #[repr(C, usize)]
        #input_vis enum #ref_ident #ref_impl_generics #ref_where_clause {
            #(#ref_variants)*
        }

        impl #impl_generics ::enum_ptr::CompactBorrow for #original_type #where_clause {
            type Target<'enum_ptr> = #ref_ident #ref_ty_generics
            where
                Self: 'enum_ptr;

            #[inline]
            fn borrow(compact: &::enum_ptr::Compact<Self>) -> Self::Target<'_> {
                unsafe {
                    compact.map_ref(|tmp| match tmp {
                        #(#match_arms)*
                    })
                }
            }
        }
    }
    .into()
}

pub fn gen_borrow_mut(input: &Input, conf: &BorrowConf) -> TokenStream {
    let input_ident = &input.ident;
    let input_vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let original_type = quote!(#input_ident #ty_generics);

    let mut ref_generics = input.generics.clone();
    ref_generics.params.insert(0, parse_quote!('enum_ptr));
    match &mut ref_generics.where_clause {
        Some(w) => w.predicates.push(parse_quote!(#original_type: 'enum_ptr)),
        none => *none = parse_quote!(where #original_type: 'enum_ptr),
    }
    let (ref_impl_generics, ref_ty_generics, ref_where_clause) = ref_generics.split_for_impl();
    let ref_ident = match &conf.name {
        Some(name) => format_ident!("{name}"),
        None => format_ident!("{input_ident}RefMut"),
    };
    let ref_derive = conf.derive.as_ref().map(|d| quote!(#[#d]));

    let ast::Data::Enum(variants) = &input.data else { unreachable!() };
    let mut ref_variants = Vec::new();
    let mut match_arms = Vec::new();
    for variant in variants {
        let variant_ident = &variant.ident;
        let field_type = &variant.fields.iter().next().unwrap().ty;
        let skip = variant.skip.is_present() || variant.skip_borrow_mut.is_present();
        if !skip {
            ref_variants.push(quote! {
                #variant_ident(<#field_type as ::enum_ptr::FieldDerefMut>::Target<'enum_ptr>),
            });
            match_arms.push(quote! {
                Self::#variant_ident(inner) => Self::Target::#variant_ident(
                    ::enum_ptr::FieldDerefMut::force_deref_mut(inner)
                ),
            });
        } else {
            ref_variants.push(quote! {
                #variant_ident(::core::marker::PhantomData<*const #field_type>),
            });
            match_arms.push(quote! {
                Self::#variant_ident(_) => Self::Target::#variant_ident(
                    ::core::marker::PhantomData
                ),
            });
        }
    }

    quote! {
        #ref_derive
        #[repr(C, usize)]
        #input_vis enum #ref_ident #ref_impl_generics #ref_where_clause {
            #(#ref_variants)*
        }

        impl #impl_generics ::enum_ptr::CompactBorrowMut for #original_type #where_clause {
            type Target<'enum_ptr> = #ref_ident #ref_ty_generics
            where
                Self: 'enum_ptr;

            #[inline]
            fn borrow_mut(compact: &mut ::enum_ptr::Compact<Self>) -> Self::Target<'_> {
                unsafe {
                    compact.map_mut(|tmp| match tmp {
                        #(#match_arms)*
                    })
                }
            }
        }
    }
    .into()
}
