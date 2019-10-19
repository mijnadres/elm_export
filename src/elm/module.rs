//! Modeling Elm's `Module`

use super::super::representation::Representation;
use super::definition::Definition;
use std::io::Write;
use syn::Result;
use syn::parse::{Parse, ParseStream};

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
    fn write_representation(&self, writer: &mut Write) -> std::io::Result<()> {
        write!(writer, "module {} exposing (..)\n", self.name)?;

        for definition in &self.definitions {
            write!(writer, "\n\n")?;
            definition.write_representation(writer)?
        }

        Ok(())
    }
}

impl Parse for Module {
    fn parse(input: ParseStream) -> Result<Self> {
        let module = Module::new("TODO");

        return Ok(module)
    }
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
