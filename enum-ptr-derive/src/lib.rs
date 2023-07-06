use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields};

fn error(span: impl Spanned, message: impl Display) -> TokenStream {
    syn::Error::new(span.span(), message)
        .to_compile_error()
        .into()
}

fn enum_ptr_derive(input: TokenStream, is_copy: bool) -> TokenStream {
    let DeriveInput {
        attrs,
        ident: enum_ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    if !attrs.contains(&parse_quote!(#[repr(C, usize)])) {
        return error(enum_ident, "EnumPtr requires `#[repr(C, usize)]`");
    }

    let variants = match data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => unreachable!(), // `#[repr(C, usize)]` implies enum
    };
    let min_align = variants.len().next_power_of_two();
    let tag_mask = min_align - 1;

    let mut asserts = Vec::new();

    for variant in &variants {
        if !matches!(variant.fields, Fields::Unnamed(_)) {
            return error(variant, "EnumPtr only supports unnamed variant");
        }
        if variant.fields.len() != 1 {
            return error(variant, "EnumPtr only supports single field");
        }
        // TODO: support discriminant later
        if variant.discriminant.is_some() {
            return error(variant, "EnumPtr doesn't support discriminants");
        }
        let variant_ident = &variant.ident;
        let field_type = &variant.fields.iter().next().unwrap().ty;
        let assert_msg = format!("`{enum_ident}::{variant_ident}` has no enough alignment");
        // TODO: change to static asserts when available (one problem is generic variables)
        asserts.push(quote! {
            assert!(
                <#field_type as ::enum_ptr::Aligned>::ALIGNMENT >= #min_align,
                #assert_msg
            );
        });
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let original_type = quote!(#enum_ident #ty_generics);
    let compact_type = if is_copy {
        quote!(::enum_ptr::CompactCopy<#original_type>)
    } else {
        quote!(::enum_ptr::Compact<#original_type>)
    };

    quote! {
        impl #impl_generics From<#original_type> for #compact_type #where_clause {
            #[inline]
            fn from(value: #original_type) -> Self {
                use ::core::mem::{transmute, transmute_copy, ManuallyDrop};
                use ::enum_ptr::PtrRepr;

                #(#asserts)*

                let PtrRepr(tag, ptr) = unsafe { transmute_copy(&ManuallyDrop::new(value)) };
                unsafe { transmute(ptr.wrapping_add(tag)) }
            }
        }

        impl #impl_generics From<#compact_type> for #original_type #where_clause {
            #[inline]
            fn from(value: #compact_type) -> Self {
                use ::core::mem::{transmute, transmute_copy};
                use ::enum_ptr::PtrRepr;

                let data: *const u8 = unsafe { transmute(value) };
                let tag = data as usize & #tag_mask;
                let ptr = data.wrapping_sub(tag);
                unsafe { transmute_copy(&PtrRepr(tag, ptr)) }
            }
        }
    }
    .into()
}

#[proc_macro_derive(EnumPtr)]
pub fn enum_ptr(input: TokenStream) -> TokenStream {
    enum_ptr_derive(input, false)
}

#[proc_macro_derive(EnumPtrCopy)]
pub fn enum_ptr_copy(input: TokenStream) -> TokenStream {
    enum_ptr_derive(input, true)
}
