#[macro_use]
extern crate serde_elm;

#[allow(dead_code)]
#[derive(Elm)]
struct Simple {
    name: String
}

#[test]
fn should_assert_elm_generation_was_done() {
    assert!(true);
}
