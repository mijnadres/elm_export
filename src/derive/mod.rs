use super::elm::Module;
use super::representation::Representation;
use std::env;
use std::fs::{create_dir, File};
use std::io::{self, BufWriter};

pub fn generate_elm(module: &Module) -> Result<(), Error> {
    let mut path = env::current_dir()?;
    path.push("generated");
    if path.exists() && path.is_file() {
        return Err(Error::GeneratedExistButIsNotADirectory);
    }
    if !path.exists() {
        create_dir(path.clone())?;
    }
    let name = &module.name;
    path.push(format!("{}.elm", name));

    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    module.write_representation(w)?;
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    GeneratedExistButIsNotADirectory,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }

}