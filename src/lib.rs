extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod error;
mod ser;
mod de;

pub use error::{Result, Error};