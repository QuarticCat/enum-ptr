mod gen;
mod input;
mod validate;

use gen::*;
use input::*;
use validate::*;

use darling::{Error, FromDeriveInput};
use proc_macro::TokenStream;

fn enum_ptr_inner(input: &syn::DeriveInput) -> Result<TokenStream, Error> {
    let input = Input::from_derive_input(input)?;

    validate_input(&input)?;

    let mut output = gen_basic(&input);
    if input.copy.is_present() {
        output.extend(gen_copy(&input));
    }

    Ok(output)
}

#[proc_macro_derive(EnumPtr, attributes(enum_ptr))]
pub fn enum_ptr(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match enum_ptr_inner(&input) {
        Ok(output) => output,
        Err(err) => err.write_errors().into(),
    }
}
