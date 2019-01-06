# serde_elm
Provides easy communication between [Elm][elm] and [Rust][rust] by leveraging serde.

## Goal
The goal of this project is to provide a bridge between Elm and Rust. We
envision a workflow where models are created in Rust, are analyzed by [syn][]
and compiled to Elm source files which are serialized by [serde][]. 

This would allow easy communication between an Elm front-end and a Rust backend
by correctly serializing and deserializing data structures on both ends of the
pipeline. 

## Usage
Notice that at the moment some of this is dreamcode.

Lets say we have some models in Rust.

```rust
enum Message {
  FriendRequest(User),
  Message(User, String),
}

struct User {
  name: String
}
```

We want to generated the corresponding models in Elm. For this we need a
dependency on the `serde_elm` crate. Include the following line in your
`Cargo.toml`. 

```toml
[dependencies]
serde_elm = "0.1.0"
```

Next we need to make our project aware of the crate and the functionality it
exposes. Import it in either `main.rs` or `lib.rs`. Don't forget to annotate the
import with the `macro_use` annotation. 

```rust
#[macro_use]
extern crate serde_elm;
```

Now it is time to derive the corresponding models in Elm. Annotate the models
with `derive(Elm)`. 

```rust
#[derive(Elm)]
enum Message {
  FriendRequest(User),
  Message(User, String),
}

#[derive(Elm)]
struct User {
  name: String
}
```

Now every time your models change and you compile your code the corresponding
Elm models will be generated. You can find them in the `generated` directory in
your projects root. The are called after the corresponding Rust definitions.
I.e. `Message.elm` and `User.elm` in this case. 

```elm
module Message exposing (..)

type Message =
    FriendRequest User
  | Message User String
```

```elm
module User exposing (..)

type alias User = 
  {
    name: String
  }
```

### Flavor of Elm
[`Json.Encode`][elm-encode] and [`Json.Decode`][elm-decode] are core Elm
packages responsible for convert Elm values into and from JSON. Anyone who
creates decoders quickly discovers
[NoRedInk/elm-decode-pipeline][elm-decode-pipeline]. It is this flavor of
decoding that we will targeting with this project. 

## Roadmap
In order to make the above code happen the following tasks need to be fulfilled.

* [ ] Generate Elm models from Rust models.
* [ ] Generate Elm decoders from Rust models.
* [ ] Generate Elm encoders from Rust models.

### Nice to haves
Although not strictly necessary, the following items are nice to have. No
promises are made when, if at all, these are implemented.

* [ ] Have proper import statements in the Elm code. 

## Contributing
Check out the [contribution guideline][contributing] if you want to contribute.

[elm]: http://elm-lang.org/
[rust]: https://www.rust-lang.org/en-US/
[syn]: https://github.com/dtolnay/syn
[serde]: https://serde.rs/
[contributing]: https://github.com/mijnadres/serde_elm/blob/master/CONTRIBUTING.md
[elm-encode]: http://package.elm-lang.org/packages/elm-lang/core/5.1.1/Json-Encode
[elm-decode]: http://package.elm-lang.org/packages/elm-lang/core/5.1.1/Json-Decode
[elm-decode-pipeline]: http://package.elm-lang.org/packages/NoRedInk/elm-decode-pipeline/3.0.0
