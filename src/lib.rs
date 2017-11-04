//! Provides easy communication between [Elm](http://elm-lang.org/) and
//! [Rust](https://www.rust-lang.org/en-US/) by leveraging
//! [serde](https://serde.rs/).
//!
//! ## Usage
//! Notice that at the moment som of this is dreamcode.
//!
//! Lets say we have some models in Rust.
//!
//! ```rust
//! enum Message {
//!   FriendRequest(User),
//!   Message(User, String),
//! }
//!
//! struct User {
//!   name: String
//! }
//! ```
//!
//! We want to generated the corresponding models in Elm. For this we need a
//! dependency on the `serde_elm` crate. Include the following line in your
//! `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! serde_elm = "0.1.0"
//! ```
//!
//! Next we need to make our project aware of the crate and the functionality it
//! exposes. Import it in either `main.rs` or `lib.rs`. Don't forget to annotate the
//! import with the `macro_use` annotation.
//!
//! ```text
//! #[macro_use]
//! extern crate serde_elm;
//! ```
//!
//! Now it is time to derive the corresponding models in Elm. Annotate the models
//! with `derive(Elm)`.
//!
//! ```text
//! #[derive(Elm)]
//! enum Message {
//!   FriendRequest(User),
//!   Message(User, String),
//! }
//!
//! #[derive(Elm)]
//! struct User {
//!   name: String
//! }
//! ```
//!
//! Now every time your models change and you compile your code the corresponding
//! Elm models will be generated. You can find them in the `generated` directory in
//! your projects root. The are called after the corresponding Rust definitions.
//! I.e. `Message.elm` and `User.elm` in this case.
//!
//! ```elm
//! module Message exposing (..)
//!
//! type Message =
//!     FriendRequest User
//!   | Message User String
//! ```
//!
//! ```elm
//! module User exposing (..)
//!
//! type alias User =
//!   {
//!     name: String
//!   }
//! ```
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

mod representation;
mod elm;
mod derive;

/// Marker trait that allows to tie in the procedural macro tool chain.
trait Elm {}

/// Writes Elm model, serializers and deserializers to the `generated`
/// directory.
#[proc_macro_derive(Elm)]
pub fn generate_elm(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();

    derive::generate_elm(&ast);

    empty_token_stream()
}

fn empty_token_stream() -> TokenStream {
    "".parse().unwrap()
}
