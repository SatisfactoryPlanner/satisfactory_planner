use std::{error, fmt::Display};

#[derive(Debug)]
pub enum ErrorCode {}

impl Display for ErrorCode {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.code, f)
    }
}

impl error::Error for Error {}
