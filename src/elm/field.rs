use std::io::{Write, Result};
use super::super::representation::Representation;
use super::Type;

pub struct Field {
    name: String,
    field_type: Type,
}

impl Field {
    pub fn new<S>(name: S, field_type: Type) -> Field where S: Into<String> {
        Field { name: name.into(), field_type }
    }
}

impl Representation for Field {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        write!(writer, "\t{}: ", self.name)?;
        self.field_type.write_representation(writer)
    }
}
