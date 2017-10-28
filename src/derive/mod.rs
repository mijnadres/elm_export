extern crate proc_macro;

extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Elm)]
pub fn generate_elm(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();

    generate_elm(&ast);

    input
}

fn generate_elm(ast: &syn::DeriveInput) {
    // do nothing
}
