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

    pub fn define(&mut self, definition: Definition) {
        self.definitions.push(definition);
    }
}

impl Representation for Module {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        write!(writer, "module {} exposing (..)\n", self.name)?;

        for definition in &self.definitions {
            write!(writer, "\n\n")?;
            definition.write_representation(writer)?
        }

        Ok(())
    }
}

pub enum Definition {
    Record(String),
    Enum(String),
    Function(String),
}

impl Representation for Definition {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        match *self {
            Definition::Record(ref name) => {
                write!(writer, "type alias {}", name)
            },

            Definition::Enum(ref name) => {
                write!(writer, "type {}", name)
            },

            Definition::Function(ref name) => {
                write!(writer, "{} =", name)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::representation::Representation;
    use super::*;

    #[test]
    fn empty_module_should_write_it_self() {
        let mut output = Vec::new();
        let module = Module::new("Test");

        module.write_representation(&mut output).unwrap();

        assert_eq!(output[0..25], b"module Test exposing (..)"[..]);
    }

    #[test]
    fn module_with_record_should_write_it_self() {
        let mut output = Vec::new();
        let mut module = Module::new("Test");
        module.define(Definition::Record(String::from("TestRecord")));

        module.write_representation(&mut output).unwrap();

        assert_eq!(output[28..49], b"type alias TestRecord"[..]);
    }

    #[test]
    fn module_with_enum_should_write_it_self() {
        let mut output = Vec::new();
        let mut module = Module::new("Test");
        module.define(Definition::Enum(String::from("TestEnum")));

        module.write_representation(&mut output).unwrap();

        assert_eq!(output[28..41], b"type TestEnum"[..]);
    }

    #[test]
    fn module_with_function_should_write_it_self() {
        let mut output = Vec::new();
        let mut module = Module::new("Test");
        module.define(Definition::Function(String::from("TestFunction")));

        module.write_representation(&mut output).unwrap();

        assert_eq!(output[28..42], b"TestFunction ="[..]);
    }
}
