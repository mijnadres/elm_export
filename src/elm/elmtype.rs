use std::io::{Write, Result};
use super::super::representation::Representation;

pub enum Type {
    String,
    Bool,
    Int,
    Float,
    Char,
    Unknown,
}

impl Representation for Type {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        let type_representation = match *self {
            Type::String => "String",
            Type::Bool => "Bool",
            Type::Int => "Int",
            Type::Float => "Float",
            Type::Char => "Char",
            Type::Unknown => "UnknownType",
        };
        write!(writer, "{},\n", type_representation)
    }
}
