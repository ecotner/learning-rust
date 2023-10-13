use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str::{self, Utf8Error};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    // an associated function (aka static method) to create a Request form a byte array
    fn from_byte_array(buf: &[u8]) -> Result<Self, ParseError> {
        unimplemented!();
    }
}

// this allows us to convert byte slice to Request struct
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        unimplemented!();
    }
}

// set of possible errors we could encounter when trying to parse a request string
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol, // we'll only support HTTP/1.1
    InvalidMethod, // if we get a request that has an invalid HTTP verb
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// method to return string interpretation of errors
impl ParseError {
    fn message(&self) -> &str {
        return match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl Error for ParseError {} // trait for creating custom error

// required to implement Display trait if Error is implemented
impl Display for ParseError {
    // this basically just formats a message to display the text of the error
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// required to implement Debug trait if Error is implemented
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
