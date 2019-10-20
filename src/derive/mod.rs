use crate::elm::Module;
use crate::representation::Representation;
use std::env;
use std::fs::{create_dir, File};
use std::io::{self, BufWriter};

pub fn generate_elm<M>(input: M) -> Result<(), Error> where M : Into<Module> {
    let mut path = env::current_dir()?;
    path.push("generated");
    if path.exists() && path.is_file() {
        return Err(Error::GeneratedExistButIsNotADirectory);
    }
    if !path.exists() {
        create_dir(path.clone())?;
    }
    let module = input.into();
    path.push(format!("{}.elm", &module.name));

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