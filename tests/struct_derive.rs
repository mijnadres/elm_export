#[macro_use]
extern crate serde_elm;

use std::env;

#[allow(dead_code)]
#[derive(Elm)]
struct Simple {
    name: String
}

#[test]
fn should_assert_elm_generation_was_done() {
    let mut path = env::current_dir().unwrap();
    path.push("generated");
    path.push("Simple.elm");

    assert!(path.exists());
    assert!(path.is_file());
}
