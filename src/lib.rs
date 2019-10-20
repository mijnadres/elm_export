//! Provides easy communication between [Elm](http://elm-lang.org/) and
//! [Rust](https://www.rust-lang.org/en-US/) by leveraging
//! [syn](https://crates.io/crates/syn).
//!
//! ## Usage
//! Notice that at the moment some of this is dreamcode.
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
//! dependency on the `elm_export` crate. Include the following line in your
//! `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! elm_export = "0.1.0"
//! ```
//!
//! Next we need to make our project aware of the crate and the functionality it
//! exposes. Import it in either `main.rs` or `lib.rs`. Don't forget to annotate the
//! import with the `macro_use` annotation.
//!
//! ```text
//! #[macro_use]
//! extern crate elm_export;
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

mod derive;
mod elm;
mod representation;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Marker trait that allows to tie in the procedural macro tool chain.
trait Elm {}

/// Writes Elm model, serializers and deserializers to the `generated`
/// directory.
#[proc_macro_derive(Elm)]
pub fn generate_elm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive::generate_elm(input).expect("to be able to generate elm module");

    empty_token_stream()
}

fn empty_token_stream() -> TokenStream {
    "".parse().unwrap()
}
