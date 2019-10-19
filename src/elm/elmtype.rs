//! Modeling Elm's types.

use super::super::representation::Representation;
use std::convert::From;
use std::io::{Result, Write};

/// The different base types of Elm
pub enum Type {
    String,
    Bool,
    Int,
    Float,
    Char,
    Unknown,
}

impl Representation for Type {
    fn write_representation(&self, writer: &mut dyn Write) -> Result<()> {
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

impl<'a> From<&'a str> for Type {
    fn from(input: &str) -> Self {
        match input {
            "String" => Type::String,
            "u8" => Type::Int,
            "i8" => Type::Int,
            "u16" => Type::Int,
            "i16" => Type::Int,
            "u32" => Type::Int,
            "i32" => Type::Int,
            "u64" => Type::Int,
            "i64" => Type::Int,
            "usize" => Type::Int,
            "isize" => Type::Int,
            "f32" => Type::Float,
            "f64" => Type::Float,
            "bool" => Type::Bool,
            "char" => Type::Char,
            _ => Type::Unknown,
        }
    }
}
