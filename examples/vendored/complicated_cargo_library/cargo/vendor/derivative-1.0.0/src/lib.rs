extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod ast;
mod attr;
mod bound;
mod clone;
mod cmp;
mod debug;
mod default;
mod hash;
mod matcher;
mod utils;

use proc_macro::TokenStream;

fn derive_impls(input: &ast::Input) -> Result<quote::Tokens, String> {
    let mut tokens = quote::Tokens::new();

    if input.attrs.clone.is_some() {
        tokens.append(&clone::derive_clone(input).to_string());
    }
    if input.attrs.copy.is_some() {
        tokens.append(&try!(clone::derive_copy(input)).to_string());
    }
    if input.attrs.debug.is_some() {
        tokens.append(&debug::derive(input).to_string());
    }
    if let Some(ref default) = input.attrs.default {
        tokens.append(&default::derive(input, default).to_string());
    }
    if input.attrs.eq.is_some() {
        tokens.append(&cmp::derive_eq(input).to_string());
    }
    if input.attrs.hash.is_some() {
        tokens.append(&hash::derive(input).to_string());
    }
    if input.attrs.partial_eq.is_some() {
        tokens.append(&try!(cmp::derive_partial_eq(input)).to_string());
    }

    Ok(tokens)
}

#[cfg_attr(not(test), proc_macro_derive(Derivative, attributes(derivative)))]
pub fn derivative(input: TokenStream) -> TokenStream {
    fn detail(input: TokenStream) -> Result<TokenStream, String> {
        let input = try!(syn::parse_macro_input(&input.to_string()));
        let parsed = try!(ast::Input::from_ast(&input));
        let output = try!(derive_impls(&parsed));
        Ok(output.to_string().parse().unwrap())
    }

    match detail(input) {
        Ok(output) => output,
        Err(e) => panic!(e),
    }
}
