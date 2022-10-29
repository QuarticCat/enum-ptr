use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumPointer)]
pub fn enum_pointer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let derived_ident = format_ident!("Compact{}", ident);
    let generics = input.generics;

    quote! {
        struct #derived_ident #generics {
            data: usize,
            phantom: ::core::marker::PhantomData<#ident #generics>,
        }

        impl #generics From<#derived_ident #generics> for #ident #generics {
            fn from(other: #derived_ident #generics) -> Self {
                unimplemented!()
            }
        }

        impl #generics From<#ident #generics> for #derived_ident #generics {
            fn from(other: #ident #generics) -> Self {
                unimplemented!()
            }
        }
    }
    .into()
}
