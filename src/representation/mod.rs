use std::io::{Result, Write};

/// Write an Elm representation of a certain type to a `writer`.
pub trait Representation {
    fn write_representation(&self, writer: &mut dyn Write) -> Result<()>;
}
