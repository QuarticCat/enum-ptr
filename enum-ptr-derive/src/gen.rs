use darling::ast;
use proc_macro::TokenStream;
use quote::quote;

use crate::Input;

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
        // TODO: change to static asserts when available (one problem is generic variables)
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
