# serde_elm
Provides easy communication between [Elm][elm] and [Rust][rust] by leveraging serde.

## Goal
The goal of this project is to provide a bridge between Elm and Rust. We envision a workflow where models are created in Rust, are analysed by [serde][]and compiled to Elm source files.

This would allow easy communication between an Elm front-end and a Rust backend by correctly serializing and deserializing data structures on both ends of the pipeline.

## Contributing
Check out the [contribution guideline][contributing] if you want to contribute.

[elm]: http://elm-lang.org/
[rust]: https://www.rust-lang.org/en-US/
[serde]: https://serde.rs/
[contributing]: https://github.com/mijnadres/serde_elm/blob/master/CONTRIBUTING.md
