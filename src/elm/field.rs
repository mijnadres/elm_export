use std::io::{Write, Result};
use super::super::representation::Representation;

pub struct Field {
    name: String
}

impl Field {
    pub fn new<S>(name: S) -> Field where S: Into<String> {
        Field { name: name.into() }
    }
}

impl Representation for Field {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        write!(writer, "\t{},\n", self.name)
    }
}
