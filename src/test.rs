use super::error::{test_error, Result};

pub struct Test<T> {
    message: String,
    test: Box<Fn(&T) -> Result<bool>>,
}

impl<T> Test<T> {
    pub fn new<M: Into<String>, V: Fn(&T) -> Result<bool> + 'static>(message: M, test: V) -> Self {
        Test {
            message: message.into(),
            test: Box::new(test),
        }
    }

    pub fn check(&self, label: &str, value: &T) -> Result<()> {
        match (self.test)(value)? {
            true => Ok(()),
            false => Err(test_error(self.message.replace("<label>", label))),
        }
    }
}
