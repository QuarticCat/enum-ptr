use std::fmt::Display;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataEnum, DeriveInput, Fields, FieldsUnnamed,
};

fn error(span: impl Spanned, message: impl Display) -> TokenStream {
    syn::Error::new(span.span(), message)
        .to_compile_error()
        .into()
}

#[proc_macro_derive(EnumPointer)]
pub fn enum_pointer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let derived_ident = format_ident!("Compact{}", ident);
    let generics = input.generics;

    let asserts = match input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            // TODO: check #[repr(C, usize)]

            let mut asserts = quote! {
                assert!(
                    ::core::mem::size_of::<#ident>() == 2 * ::core::mem::size_of::<usize>(),
                    concat!("`", stringify!(#ident), "` should be 2 pointers wide")
                );
            };

            let num_variants = variants.len();
            for variant in variants {
                match variant.fields {
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        if unnamed.len() != 1 {
                            return error(unnamed, "EnumPointer doesn't support multiple fields");
                        }
                        let variant_ident = variant.ident;
                        let field = unnamed.first().unwrap();
                        asserts.extend(quote!(
                            assert!(
                                <#field as ::enum_pointer::Compactable>::ALIGN >= #num_variants,
                                concat!("`", stringify!(#ident), "::", stringify!(#variant_ident), "` has no enough alignment")
                            );
                        ));
                    }
                    named @ Fields::Named(_) => {
                        return error(named, "EnumPointer doesn't support named fields");
                    }
                    _ => {}
                }
            }

            asserts
        }
        // TODO: better error span
        _ => return error(ident, "EnumPointer only supports enums"),
    };

    quote! {
        struct #derived_ident #generics {
            data: usize,
            phantom: ::core::marker::PhantomData<#ident #generics>,
        }

        impl #generics #derived_ident #generics {
            const _CHECK: () = {
                #asserts
            };
        }

        impl #generics From<#derived_ident #generics> for #ident #generics {
            fn from(other: #derived_ident #generics) -> Self {
                unimplemented!()
            }
        }

        impl #generics From<#ident #generics> for #derived_ident #generics {
            fn from(other: #ident #generics) -> Self {
                let _ = Self::_CHECK; // trigger static asserts
                unimplemented!()
            }
        }
    }
    .into()
}
