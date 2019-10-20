//! Data structures modeling Elm's type system.

mod module;
mod definition;
mod field;
mod elmtype;

use syn::{DeriveInput, Data, DataStruct, DataEnum};

pub use self::module::Module;
pub use self::definition::Definition;
pub use self::field::Field;
pub use self::elmtype::Type;

pub fn transform(input: DeriveInput) -> Module {
    let name = input.ident.to_string();
    let mut module = Module::new(&name);

    let definition = match input.data {
        Data::Struct(data_struct) => define_struct(&name, data_struct),

        Data::Enum(data_enum) => define_enum(&name, data_enum),

        _ => unimplemented!(),
    };
    module.define(definition);

    module
}

fn define_struct<S>(name : S, _data_struct: DataStruct) -> Definition where S: Into<String> {
    let definition = Definition::Record(name);

    definition
}

fn define_enum<S>(name : S, _data_enum: DataEnum) -> Definition where S: Into<String> {
    let definition = Definition::Record(name);

    definition
}