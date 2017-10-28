#[macro_use]
extern crate quote;

use std::str::FromStr;

#[test]
fn should_quote_arbitrary_text() {
    let stream = quote!(This is arbitrary text);

    let representation = stream.to_string();

    assert_eq!(String::from_str("This is arbitrary text").unwrap(), representation);
}

#[test]
fn should_capture_variables_in_context() {
    let name = quote!(World);
    let stream = quote!(Hello, #name);

    let representation = stream.to_string();

    assert_eq!(String::from_str("Hello , World").unwrap(), representation);
}
