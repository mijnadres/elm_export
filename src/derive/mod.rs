extern crate syn;

use std::env;
use std::fs::{File, create_dir};
use std::io::{Write, BufWriter};
use syn::DeriveInput;

pub fn generate_elm(ast: &DeriveInput) {
    let mut path = env::current_dir().unwrap();
    path.push("generated");
    if path.exists() && path.is_file() { panic!("expecting \"generated\" to be a directory"); }
    if !path.exists() { create_dir(path.clone()).expect("problem creating \"generated\" directory") }
    let name = &ast.ident;
    path.push(format!("{}.elm", name));

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    w.write("Hello, World!".as_bytes()).unwrap();
}
