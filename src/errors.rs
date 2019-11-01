use std::fmt;
use std::error;
use std::str;

use lopdf;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lopdf(lopdf::Error),
    Utf8(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Lopdf(ref err) => write!(f, "pdf error: {}", err),
            Error::Utf8(ref err) => write!(f, "utf8 error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Lopdf(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
        }
    }
}

impl From<lopdf::Error> for Error {
    fn from(err: lopdf::Error) -> Error {
        Error::Lopdf(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}
