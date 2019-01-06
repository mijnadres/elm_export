//! Data structures modeling Elm's type system.

mod definition;
mod elmtype;
mod field;
mod module;

pub use self::definition::Definition;
pub use self::elmtype::Type;
pub use self::field::Field;
pub use self::module::Module;
