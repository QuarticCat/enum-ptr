use std::fmt::Display;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
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
        vis,
        ident: enum_ident,
        generics,
        data,
    } = parse_macro_input!(input);

    if !attrs.contains(&parse_quote!(#[repr(C, usize)])) {
        return error(enum_ident, "EnumPtr requires `#[repr(C, usize)]`");
    }

    let new_enum_ident = format_ident!("Compact{enum_ident}");
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
                    <#field_type as ::enum_ptr::Compactable>::Pointee
                >() >= #min_align,
                #assert_msg
            );
        });
    }

    // For unit variants, the latter `usize`s are uninitialized.
    // So we cannot simply do `tag | ptr`.
    let get_data = if unit_variants.is_empty() {
        quote! {{
            let [tag, ptr]: [usize; 2] = unsafe { ::core::mem::transmute(other) };
            unsafe { ::core::mem::transmute(tag | ptr) }
        }}
    } else {
        quote! {{
            match other {
                #(#enum_ident::#unit_variants)|* => {
                    let tag = unsafe{ *(&other as *const _ as *const usize) };
                    unsafe { ::core::mem::transmute(tag) }
                }
                _ => {
                    let [tag, ptr]: [usize; 2] = unsafe { ::core::mem::transmute(other) };
                    unsafe { ::core::mem::transmute(tag | ptr) }
                }
            }
        }}
    };

    quote! {
        #[repr(transparent)]
        #vis struct #new_enum_ident #generics {
            data: ::enum_ptr::Private<usize>,
            phantom: ::core::marker::PhantomData<#enum_ident #generics>,
        }

        impl #generics Drop for #new_enum_ident #generics {
            fn drop(&mut self) {
                let this: Self = unsafe { ::core::mem::transmute_copy(self) };
                let _ = #enum_ident::from(this);
            }
        }

        impl #generics From<#new_enum_ident #generics> for #enum_ident #generics {
            fn from(other: #new_enum_ident #generics) -> Self {
                let data: usize = unsafe { ::core::mem::transmute(other) };
                let tag_ptr = [data & #tag_mask, data & !#tag_mask];
                unsafe { ::core::mem::transmute(tag_ptr) }
            }
        }

        impl #generics From<#enum_ident #generics> for #new_enum_ident #generics {
            fn from(other: #enum_ident #generics) -> Self {
                #(#asserts)*
                Self {
                    data: #get_data,
                    phantom: ::core::marker::PhantomData,
                }
            }
        }
    }
    .into()
}
