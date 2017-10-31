extern crate syn;

use std::env;
use std::fs::{File, create_dir};
use std::io::BufWriter;
use syn::DeriveInput;
use super::representation::Representation;
use super::elm::Module;

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
    Module::new(name.to_string())
}
