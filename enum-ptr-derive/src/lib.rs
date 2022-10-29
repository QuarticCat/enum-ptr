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

    let mut asserts = vec![quote! {
        assert!(
            ::core::mem::size_of::<#ident>() == 2 * ::core::mem::size_of::<usize>(),
            concat!("`", stringify!(#ident), "` should be 2 pointers wide")
        );
    }];

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
                if let Some((_, expr)) = variant.discriminant {
                    return error(expr, "EnumPtr doesn't support discriminant values");
                }

                match variant.fields {
                    Fields::Named(FieldsNamed { named: fields, .. })
                    | Fields::Unnamed(FieldsUnnamed {
                        unnamed: fields, ..
                    }) => {
                        if fields.len() != 1 {
                            return error(fields, "EnumPtr doesn't support multiple fields");
                        }

                        let variant_ident = variant.ident;
                        let field = fields.first().unwrap();
                        asserts.push(quote!(
                            assert!(
                                <#field as ::enum_ptr::Compactable>::ALIGN >= #min_align,
                                concat!("`", stringify!(#ident), "::", stringify!(#variant_ident), "` has no enough alignment")
                            );
                        ));
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

        impl #generics #derived_ident #generics {
            const _CHECK: () = { #(#asserts)* };
        }

        impl #generics From<#derived_ident #generics> for #ident #generics {
            fn from(other: #derived_ident #generics) -> Self {
                let tag_ptr = (other.data & #tag_mask, other.data & !#tag_mask);
                unsafe { ::core::mem::transmute::<_, Self>(tag_ptr) }
            }
        }

        impl #generics From<#ident #generics> for #derived_ident #generics {
            fn from(other: #ident #generics) -> Self {
                let _ = Self::_CHECK; // trigger static asserts
                let (tag, ptr) = unsafe { ::core::mem::transmute::<_, (usize, usize)>(other) };
                Self {
                    data: tag | ptr,
                    phantom: ::core::marker::PhantomData,
                }
            }
        }
    }
    .into()
}
