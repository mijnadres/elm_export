extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

mod derive;

/// Marker trait that allows to tie in the procedural macro tool chain.
trait Elm {}

#[proc_macro_derive(Elm)]
pub fn generate_elm(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();

    derive::generate_elm_for(&ast);

    input
}

