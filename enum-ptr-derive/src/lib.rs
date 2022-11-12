use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput};

fn error(span: impl Spanned, message: impl Display) -> TokenStream {
    syn::Error::new(span.span(), message)
        .to_compile_error()
        .into()
}

#[proc_macro_derive(EnumPtr)]
pub fn enum_ptr(input: TokenStream) -> TokenStream {
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
    let mut unit_variants = Vec::new();

    for variant in variants {
        if variant.fields.is_empty() {
            unit_variants.push(variant);
            continue;
        }

        if variant.fields.len() > 1 {
            return error(variant.fields, "EnumPtr doesn't support multiple fields");
        }

        let variant_ident = variant.ident;
        let field_type = &variant.fields.iter().next().unwrap().ty;
        let assert_msg = format!("`{enum_ident}::{variant_ident}` has no enough alignment");
        asserts.push(quote! {
            assert!(
                ::core::mem::align_of::<
                    <#field_type as ::enum_ptr::Aligned>::Pointee
                >() >= #min_align,
                #assert_msg
            );
        });
    }

    let original_type = quote!(#enum_ident #generics);
    let compact_type = quote!(::enum_ptr::Compact<#original_type>);

    // For unit variants, the latter `usize`s are uninitialized.
    // So we cannot simply do `tag | ptr`.
    let compaction = if unit_variants.is_empty() {
        quote! {
            let ::enum_ptr::PtrRepr(tag, ptr) = unsafe { ::core::mem::transmute(other) };
            unsafe { ::core::mem::transmute(ptr.wrapping_add(tag)) }
        }
    } else {
        quote! {
            match other {
                #(#enum_ident::#unit_variants)|* => {
                    let ::enum_ptr::UnitRepr(tag, _) = unsafe { ::core::mem::transmute(other) };
                    let ptr: *const u8 = ::core::ptr::null();
                    unsafe { ::core::mem::transmute(ptr.wrapping_add(tag)) }
                }
                _ => {
                    let ::enum_ptr::PtrRepr(tag, ptr) = unsafe { ::core::mem::transmute(other) };
                    unsafe { ::core::mem::transmute(ptr.wrapping_add(tag)) }
                }
            }
        }
    };

    quote! {
        impl #generics From<#original_type> for #compact_type {
            fn from(other: #original_type) -> Self {
                #(#asserts)*
                #compaction
            }
        }

        impl #generics From<#compact_type> for #original_type {
            fn from(other: #compact_type) -> Self {
                let data: *const u8 = unsafe { ::core::mem::transmute(other) };
                let tag = data as usize & #tag_mask;
                let ptr = data.wrapping_sub(tag);
                unsafe { ::core::mem::transmute(::enum_ptr::PtrRepr(tag, ptr)) }
            }
        }
    }
    .into()
}
