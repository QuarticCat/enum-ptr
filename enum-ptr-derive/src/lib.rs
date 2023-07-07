mod gen;
mod input;
mod validate;

use gen::*;
use input::*;
use validate::*;

use darling::{Error, FromDeriveInput};
use proc_macro::TokenStream;

#[proc_macro_derive(EnumPtr, attributes(enum_ptr))]
pub fn enum_ptr(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match enum_ptr_inner(&input) {
        Ok(output) => output,
        Err(err) => err.write_errors().into(),
    }
}

fn enum_ptr_inner(input: &syn::DeriveInput) -> Result<TokenStream, Error> {
    let input = Input::from_derive_input(input)?;

    validate_input(&input)?;

    let mut output = gen_basic(&input);
    if input.copy.is_present() {
        output.extend(gen_copy(&input));
    }
    if let Some(conf) = input.borrow.clone() {
        output.extend(gen_borrow(&input, &conf.unwrap_or_default()));
    }
    if let Some(conf) = input.borrow_mut.clone() {
        output.extend(gen_borrow_mut(&input, &conf.unwrap_or_default()));
    }

    Ok(output)
}
