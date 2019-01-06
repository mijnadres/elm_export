use std::io::{Write, Result};

/// Write an Elm representation of a certain type to a `writer`.
pub trait Representation {
    fn write_representation(&self, writer: &mut Write) -> Result<()>;
}
