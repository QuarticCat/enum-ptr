use std::fmt::Display;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DeriveInput, Fields,
    FieldsNamed, FieldsUnnamed,
};

fn error(span: impl Spanned, message: impl Display) -> TokenStream {
    syn::Error::new(span.span(), message)
        .to_compile_error()
        .into()
}

#[proc_macro_derive(EnumPtr)]
pub fn enum_ptr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let vis = input.vis;
    let ident = input.ident;
    let derived_ident = format_ident!("Compact{}", ident);
    let generics = input.generics;

    let mut asserts = Vec::new();
    let tag_mask;

    match input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            // Put after enum check to make testing easier
            if !input.attrs.contains(&parse_quote!(#[repr(C, usize)])) {
                return error(ident, "EnumPtr requires `#[repr(C, usize)]`");
            }

            let min_align = variants.len().next_power_of_two();
            tag_mask = min_align - 1;

            for variant in variants {
                match variant.fields {
                    Fields::Named(FieldsNamed { named: fields, .. })
                    | Fields::Unnamed(FieldsUnnamed {
                        unnamed: fields, ..
                    }) => {
                        if fields.is_empty() {
                            continue;
                        }
                        if fields.len() > 1 {
                            return error(fields, "EnumPtr doesn't support multiple fields");
                        }

                        let variant_ident = variant.ident;
                        let field_type = &fields.first().unwrap().ty;
                        let assert_msg =
                            format!("`{}::{}` has no enough alignment", ident, variant_ident);
                        asserts.push(quote! {
                            assert!(
                                ::core::mem::align_of::<
                                    <#field_type as ::enum_ptr::Compactable>::Pointee
                                >() >= #min_align,
                                #assert_msg
                            );
                        });
                    }
                    Fields::Unit => {}
                }
            }
        }
        _ => return error(ident, "EnumPtr only supports enums"),
    }

    quote! {
        #vis struct #derived_ident #generics {
            data: usize,
            phantom: ::core::marker::PhantomData<#ident #generics>,
        }

        impl #generics From<#derived_ident #generics> for #ident #generics {
            fn from(other: #derived_ident #generics) -> Self {
                let tag_ptr = ::enum_ptr::EnumRepr(
                    other.data & #tag_mask,
                    other.data & !#tag_mask,
                );
                unsafe { ::core::mem::transmute(tag_ptr) }
            }
        }

        impl #generics From<#ident #generics> for #derived_ident #generics {
            fn from(other: #ident #generics) -> Self {
                #(#asserts)*

                let ::enum_ptr::EnumRepr(tag, ptr) = unsafe { ::core::mem::transmute(other) };
                Self {
                    data: tag | ptr,
                    phantom: ::core::marker::PhantomData,
                }
            }
        }
    }
    .into()
}
