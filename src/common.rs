use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

pub struct XBFError {
    msg: String,
}

impl From<&str> for XBFError {
    fn from(value: &str) -> Self {
        XBFError {
            msg: value.to_string(),
        }
    }
}

impl From<String> for XBFError {
    fn from(value: String) -> Self {
        XBFError { msg: value }
    }
}

impl From<fmt::Error> for XBFError {
    fn from(value: fmt::Error) -> Self {
        XBFError::from(format!("{}", value))
    }
}

impl Debug for XBFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Display for XBFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for XBFError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn try_index<T>(vec: &Vec<T>, index: usize) -> Option<&T> {
    if index < vec.len() {
        Some(&vec[index])
    } else {
        None
    }
}
