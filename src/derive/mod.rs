use super::elm::Module;
use super::representation::Representation;
use std::env;
use std::fs::{create_dir, File};
use std::io::BufWriter;

pub fn generate_elm(module: &Module) {
    let mut path = env::current_dir().unwrap();
    path.push("generated");
    if path.exists() && path.is_file() {
        panic!("expecting \"generated\" to be a directory");
    }
    if !path.exists() {
        create_dir(path.clone()).expect("problem creating \"generated\" directory");
    }
    let name = &module.name;
    path.push(format!("{}.elm", name));

    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    module.write_representation(w).unwrap();
}
