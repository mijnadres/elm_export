#[macro_use]
extern crate serde_elm;

use std::env;
use std::fs::File;
use std::io::{Read, Error};
use std::path::Path;

#[allow(dead_code)]
#[derive(Elm)]
struct Simple {
    name: String,
}

#[test]
fn should_assert_elm_generation_was_done() {
	let model_name = "Simple";
	assert!(content_equal_for(model_name))
}

fn content_equal_for<S>(model_name: S) -> bool where S: Into<String> {
	let model_name = model_name.into();
	let root = env::current_dir().unwrap();
    let mut path = root.clone();
    path.push("generated");
    path.push(format!("{}.elm", model_name));

    assert!(path.exists());
    assert!(path.is_file());

    let actual = contents_of(&path).unwrap();

    let mut expected_path = root.clone();
    expected_path.push("tests");
    expected_path.push("assets");
    expected_path.push(format!("{}.test.elm", model_name));

    let expected = contents_of(&expected_path).unwrap();

    actual == expected
}

fn contents_of(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
