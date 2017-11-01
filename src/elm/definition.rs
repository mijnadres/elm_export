use std::io::{Write, Result};
use super::super::representation::Representation;

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
