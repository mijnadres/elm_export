use std::io::{Write, Result};
use super::representation::Representation;

pub struct Module {
    name: String,
}

impl Module {
    pub fn new<S>(name: S) -> Module where S: Into<String> {
        Module { name : name.into() }
    }
}

impl Representation for Module {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        write!(writer, "module {} exposing (..)", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::super::representation::Representation;
    use super::*;

    #[test]
    fn module_should_write_it_self() {
        let module = Module::new("Test");
        let mut output = Vec::new();

        module.write_representation(&mut output).unwrap();

        assert_eq!(output, b"module Test exposing (..)");
    }
}
