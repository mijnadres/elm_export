//! Modeling Elm's `Module`

use crate::representation::Representation;
use crate::elm::Definition;
use std::io::{Result, Write};
use syn::{DeriveInput, Data, DataStruct, DataEnum};

/// A module has a name and a sequence of definitions
pub struct Module {
    pub name: String,
    definitions: Vec<Definition>,
}

impl Module {
    /// Create a `Module` with a certain name.
    pub fn new<S>(name: S) -> Module
    where
        S: Into<String>,
    {
        Module {
            name: name.into(),
            definitions: Vec::new(),
        }
    }

    /// Include a `Definition` in this module.
    pub fn define(&mut self, definition: Definition) {
        self.definitions.push(definition);
    }
}

impl Representation for Module {
    fn write_representation(&self, writer: &mut dyn Write) -> Result<()> {
        write!(writer, "module {} exposing (..)\n", self.name)?;

        for definition in &self.definitions {
            write!(writer, "\n\n")?;
            definition.write_representation(writer)?
        }

        Ok(())
    }
}

impl From<DeriveInput> for Module {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident.to_string();
        let mut module = Module::new(&name);

        let definition = match input.data {
            Data::Struct(data_struct) => define_struct(&name, data_struct),

            Data::Enum(data_enum) => define_enum(&name, data_enum),

            _ => unimplemented!(),
        };
        module.define(definition);

        module
    }
}

fn define_struct<S>(name : S, _data_struct: DataStruct) -> Definition where S: Into<String> {
    let definition = Definition::Record(name);

    definition
}

fn define_enum<S>(name : S, _data_enum: DataEnum) -> Definition where S: Into<String> {
    let definition = Definition::Record(name);

    definition
}

#[cfg(test)]
mod tests {
    use super::super::super::representation::Representation;
    use super::*;

    #[test]
    fn empty_module_should_write_it_self() {
        let mut output = Vec::new();
        let module = Module::new("Test");

        module.write_representation(&mut output).unwrap();

        assert_eq!(output[0..25], b"module Test exposing (..)"[..]);
    }
}
