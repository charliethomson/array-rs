
use std::{
    fmt,
    error::Error,
};

pub struct ArrayError {
    msg: String,
} impl ArrayError {
    pub fn new<S>(msg: S) -> Self where S: ToString {
        Self {
            msg: msg.to_string()
        }
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }

    pub fn matches<S: ToString>(&self, other: S) -> bool{
        self.msg == other.to_string()
    }
}

impl Error for ArrayError { }

impl fmt::Display for ArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Debug for ArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArrayError {{ msg: {} }}", self.msg)
    }
}