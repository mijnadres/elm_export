//! Modeling a definition in Elm.

use crate::representation::Representation;
use crate::elm::Field;
use std::io::{Result, Write};
use syn::{DeriveInput, Data, DataStruct, DataEnum};

/// A `Definition` is a model of the different types one can define in Elm.
pub struct Definition {
    name: String,
    definition_type: DefinitionType,
    fields: Vec<Field>,
}

/// Model for the different possible type constructions in Elm.
pub enum DefinitionType {
    Record,
    Enum,
}

impl Definition {
    /// Create an Elm record.
    #[allow(non_snake_case)]
    pub fn Record<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Definition {
            name: name.into(),
            fields: vec![],
            definition_type: DefinitionType::Record,
        }
    }

    /// Create an Elm enumeration.
    #[allow(non_snake_case)]
    pub fn Enum<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Definition {
            name: name.into(),
            fields: vec![],
            definition_type: DefinitionType::Enum,
        }
    }

    /// add a field to our definition.
    pub fn add<F>(&mut self, field: F) where F: Into<Field> {
        self.fields.push(field.into());
    }
}

impl Representation for Definition {
    fn write_representation(&self, writer: &mut dyn Write) -> Result<()> {
        match self.definition_type {
            DefinitionType::Record => {
                write!(writer, "type alias {} = {{\n", self.name)?;
                for field in &self.fields {
                    field.write_representation(writer)?;
                }
                write!(writer, "}}\n")
            }

            DefinitionType::Enum => write!(writer, "type {}", self.name),
        }
    }
}

impl From<DeriveInput> for Definition {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident.to_string();
        match input.data {
            Data::Struct(data_struct) => define_struct(name, data_struct),

            Data::Enum(data_enum) => define_enum(name, data_enum),

            _ => unimplemented!(),
        }
    }
}

fn define_struct<S>(name : S, data_struct: DataStruct) -> Definition where S: Into<String> {
    let mut definition = Definition::Record(name);

    for field in data_struct.fields {
        definition.add(field);
    }

    definition 
}

fn define_enum<S>(name : S, data_enum: DataEnum) -> Definition where S: Into<String> {
    let mut definition = Definition::Enum(name);

    for variant in data_enum.variants {
        definition.add(variant);
    }

    definition
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
}
