use std::fmt;

#[derive(Debug)]
pub enum CheckErrorValue {
    Constant(String),
}

#[derive(Debug)]
pub struct CheckError {
    pub value: CheckErrorValue,
}

impl CheckError {
    pub fn new(value: &str) -> CheckError {
        CheckError {
            value: CheckErrorValue::Constant(value.to_owned()),
        }
    }
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            CheckErrorValue::Constant(ref s) => write!(f, "{}", s),
        }
    }
}
