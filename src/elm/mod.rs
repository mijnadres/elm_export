use std::io::{Write, Result};
use super::representation::Representation;

pub struct Module {
    name: String,
    definitions: Vec<Definition>,
}

impl Module {
    pub fn new<S>(name: S) -> Module where S: Into<String> {
        Module { name : name.into(), definitions: Vec::new() }
    }
}

impl Representation for Module {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        write!(writer, "module {} exposing (..)\n\n\n", self.name)?;

        for definition in &self.definitions {
            definition.write_representation(writer)?
        }

        Ok(())
    }
}

pub enum Definition {
    Struct,
    Enum,
    Function,
}

impl Representation for Definition {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        match *self {
            Definition::Struct => {
                write!(writer, "type alias")
            },

            Definition::Enum => {
                write!(writer, "type")
            },

            Definition::Function => {
                write!(writer, "function")
            },
        }
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

        assert_eq!(output[0..25], b"module Test exposing (..)"[..]);
    }
}
