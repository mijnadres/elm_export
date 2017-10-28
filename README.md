# serde_elm
Provides easy communication between [Elm][elm] and [Rust][rust] by leveraging serde.

## Goal
The goal of this project is to provide a bridge between Elm and Rust. We envision a workflow where models are created in Rust, are analysed by [serde][]and compiled to Elm source files.

This would allow easy communication between an Elm front-end and a Rust backend by correctly serializing and deserializing data structures on both ends of the pipeline.

## Usage
Notice that at the moment this is all dreamcode.

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

We generated the corresponding models in Elm with the following command.

```sh
generate-elm
```

Which will create the following models

```elm
type Message =
    FriendRequest User
  | Message User String

type alias User = 
  {
    name: String
  }
```

### Flavor of Elm
[`Json.Encode`][elm-encode] and [`Json.Decode`][elm-decode] are core Elm packages responsible for convert Elm values into and from JSON. Anyone who creates decoders quickly discovers [NoRedInk/elm-decode-pipeline][elm-decode-pipeline]. It is this flavor of decoding that we will targeting with this project.

## Contributing
Check out the [contribution guideline][contributing] if you want to contribute.

[elm]: http://elm-lang.org/
[rust]: https://www.rust-lang.org/en-US/
[serde]: https://serde.rs/
[contributing]: https://github.com/mijnadres/serde_elm/blob/master/CONTRIBUTING.md
[elm-encode]: http://package.elm-lang.org/packages/elm-lang/core/5.1.1/Json-Encode
[elm-decode]: http://package.elm-lang.org/packages/elm-lang/core/5.1.1/Json-Decode
[elm-decode-pipeline]: http://package.elm-lang.org/packages/NoRedInk/elm-decode-pipeline/3.0.0
