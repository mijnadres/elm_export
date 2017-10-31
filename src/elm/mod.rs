pub struct Module {
    name: String,
}

impl Module {
    fn new<S>(name: S) -> Module where S: Into<String> {
        Module { name : name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_should_write_it_self() {
        let module = Module::new("Test");
    }
}
