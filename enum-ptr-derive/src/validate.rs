use darling::{ast, Error};

use crate::Input;

pub fn validate_input(input: &Input) -> Result<(), Error> {
    let mut errors = Error::accumulator();

    if !input.attrs.contains(&syn::parse_quote!(#[repr(C, usize)])) {
        errors.push(Error::custom("missing `#[repr(C, usize)]`"))
    }

    let ast::Data::Enum(variants) = &input.data else { unreachable!() };
    for variant in variants {
        if variant.fields.len() != 1 {
            errors.push(Error::custom("expect exactly one field").with_span(&variant.ident));
        }
        // TODO: support discriminant later
        if let Some(discriminant) = &variant.discriminant {
            errors.push(Error::custom("discrinimant is unsupported").with_span(discriminant))
        }
    }

    errors.finish()
}
