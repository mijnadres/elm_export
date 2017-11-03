use std::env;
use std::fs::{File, create_dir};
use std::io::BufWriter;
use syn;
use syn::{DeriveInput, Body, VariantData, Ty, Path};
use super::representation::Representation;
use super::elm::{Module, Definition, Field, Type};

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
                        let field = field_from(field);
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

fn field_from(field: &syn::Field) -> Field {
    let field_name = field.clone().ident.unwrap();
    let field_type = match field.ty {
        Ty::Path(_, ref path) => {
            let path_type = path.segments.last().unwrap();
            match path_type.ident.to_string().as_str() {
                "String" => Type::String,
                _ => Type::Unknown,
            }
        }
        _ => Type::Unknown,
    };
    let field = Field::new(field_name.to_string(), field_type);
    field
}
