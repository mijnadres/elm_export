//! Modeling Elm fields

use crate::representation::Representation;
use crate::elm::Type;
use std::io::{Result, Write};

/// A field has a name and a type.
pub struct Field {
    name: String,
    field_type: Type,
}

impl Field {
    /// Create a field
    pub fn new<S, T>(name: S, field_type: T) -> Field
    where
        S: Into<String>,
        T: Into<Type>
    {
        Field {
            name: name.into(),
            field_type: field_type.into(),
        }
    }
}

impl Representation for Field {
    fn write_representation(&self, writer: &mut dyn Write) -> Result<()> {
        write!(writer, "\t{}: ", self.name)?;
        self.field_type.write_representation(writer)
    }
}

impl From<syn::Field> for Field {
    fn from(input: syn::Field) -> Self {
        let name = input.ident.map(|identifier| identifier.to_string()).unwrap_or("field_with_no_name".to_string());
        let field = Field::new(name, input.ty);
        field
    }
}

impl From<syn::Variant> for Field {
    fn from(input: syn::Variant) -> Self {
        let name = input.ident.to_string();
        let field = Field::new(name, Type::Unknown);
        field
    }
}