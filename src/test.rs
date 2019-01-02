use super::error::{test_error, Result};

pub struct Test<T> {
    type_: &'static str,
    message: String,
    test: Box<Fn(&T) -> Result<bool>>,
}

impl<T> Test<T> {
    pub fn new<M, F>(type_: &'static str, message: M, test: F) -> Self
    where
        M: Into<String>,
        F: Fn(&T) -> Result<bool> + 'static,
    {
        Test {
            type_,
            test: Box::new(test),
            message: message.into(),
        }
    }

    pub fn check(&self, path: &str, label: &str, value: &T) -> Result<()> {
        match (self.test)(value)? {
            true => Ok(()),
            false => Err(test_error(
                self.type_,
                path,
                &self.message.replace("<label>", label),
            )),
        }
    }
}
