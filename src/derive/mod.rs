use std::env;
use std::fs::{File, create_dir};
use std::io::BufWriter;
use syn::{DeriveInput, Body, VariantData};
use super::representation::Representation;
use super::elm::{Module, Definition, Field};

pub fn generate_elm(ast: &DeriveInput) {
    let mut path = env::current_dir().unwrap();
    path.push("generated");
    if path.exists() && path.is_file() { panic!("expecting \"generated\" to be a directory"); }
    if !path.exists() { create_dir(path.clone()).expect("problem creating \"generated\" directory") }
    let name = &ast.ident;
    path.push(format!("{}.elm", name));

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let m = module_from(ast);

    m.write_representation(w).unwrap();
}

fn module_from(ast: &DeriveInput) -> Module {
    let name = &ast.ident;
    let mut module = Module::new(name.to_string());

    let definition = definition_from(&ast.body, name.to_string());
    module.define(definition);

    module
}

fn definition_from(body: &Body, name: String) -> Definition {
    match *body {
        Body::Enum(ref variants) => {
            let definition = Definition::Enum(name);
            definition
        },
        Body::Struct(ref variant_data) => {
            let mut definition = Definition::Record(name);
            match *variant_data {
                VariantData::Struct(ref fields) => {
                    for field in fields {
                        let field_name = field.clone().ident.unwrap();
                        let field = Field::new(field_name.to_string());
                        definition.add(field)
                    }
                },
                VariantData::Tuple(ref fields) => {
                    unimplemented!()
                },
                VariantData::Unit => {
                    unimplemented!()
                },
            }
            definition
        }
    }
}
