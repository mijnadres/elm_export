use std::io::{Write, Result};
use super::super::representation::Representation;
use super::field::Field;

pub struct Definition {
    name: String,
    definition_type: DefinitionType,
    fields: Vec<Field>,
}

pub enum DefinitionType {
    Record,
    Enum,
    Function,
}

impl Definition {
    #[allow(non_snake_case)]
    pub fn Record<S>(name: S) -> Definition where S: Into<String> {
        Definition { name: name.into(), fields: vec!(), definition_type: DefinitionType::Record }
    }

    #[allow(non_snake_case)]
    pub fn Enum<S>(name: S) -> Definition where S: Into<String> {
        Definition { name: name.into(), fields: vec!(), definition_type: DefinitionType::Enum }
    }

    #[allow(non_snake_case)]
    pub fn Function<S>(name: S) -> Definition where S: Into<String> {
        Definition { name: name.into(), fields: vec!(), definition_type: DefinitionType::Function }
    }

    pub fn add(&mut self, field: Field) {
        self.fields.push(field);
    }
}


impl Representation for Definition {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        match self.definition_type {
            DefinitionType::Record => {
                write!(writer, "type alias {} = {{\n", self.name)?;
                for field in &self.fields {
                    field.write_representation(writer)?;
                }
                write!(writer, "}}\n")
            },

            DefinitionType::Enum => {
                write!(writer, "type {}", self.name)
            },

            DefinitionType::Function => {
                write!(writer, "{} =", self.name)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::representation::Representation;
    use super::*;

    #[test]
    fn record_should_write_it_self() {
        let mut output = Vec::new();
        let definition = Definition::Record(String::from("TestRecord"));

        definition.write_representation(&mut output).unwrap();

        assert_eq!(output[0..21], b"type alias TestRecord"[..]);
    }

    #[test]
    fn enum_should_write_it_self() {
        let mut output = Vec::new();
        let definition = Definition::Enum(String::from("TestEnum"));

        definition.write_representation(&mut output).unwrap();

        assert_eq!(output[0..13], b"type TestEnum"[..]);
    }

    #[test]
    fn function_should_write_it_self() {
        let mut output = Vec::new();
        let definition = Definition::Function(String::from("TestFunction"));

        definition.write_representation(&mut output).unwrap();

        assert_eq!(output[0..14], b"TestFunction ="[..]);
    }
}