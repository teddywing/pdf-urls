use std::fmt;
use std::error;
use std::str;
use std::string;

use lopdf;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lopdf(lopdf::Error),
    String(StringError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Lopdf(ref err) => write!(f, "pdf error: {}", err),
            Error::String(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Lopdf(ref err) => Some(err),
            Error::String(ref err) => Some(err),
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
        Error::String(StringError::from(err))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::String(StringError::from(err))
    }
}


#[derive(Debug)]
pub enum StringError {
    StrUtf8(str::Utf8Error),
    StringUtf8(string::FromUtf8Error),
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StringError::StrUtf8(ref err) => write!(f, "utf8 error: {}", err),
            StringError::StringUtf8(ref err) => write!(f, "utf8 error: {}", err),
        }
    }
}

impl error::Error for StringError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            StringError::StrUtf8(ref err) => Some(err),
            StringError::StringUtf8(ref err) => Some(err),
        }
    }
}

impl From<str::Utf8Error> for StringError {
    fn from(err: str::Utf8Error) -> StringError {
        StringError::StrUtf8(err)
    }
}

impl From<string::FromUtf8Error> for StringError {
    fn from(err: string::FromUtf8Error) -> StringError {
        StringError::StringUtf8(err)
    }
}
