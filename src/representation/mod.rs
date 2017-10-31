use std::io::{Write, Result};

pub trait Representation {
    fn write_representation(&self, writer: &mut Write) -> Result<()>;
}
