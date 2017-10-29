extern crate syn;

use std::env;
use std::fs::File;
use std::io::{Write, BufWriter};
use syn::DeriveInput;

pub fn generate_elm(ast: &DeriveInput) {
    let mut path = env::current_dir().unwrap();
    let name = &ast.ident;
    path.push(format!("generated/{}.elm", name));

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    w.write("Hello, World!".as_bytes()).unwrap();
}
