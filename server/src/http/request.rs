use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str::{self, Utf8Error};
use super::QueryString;

// this is generic over a lifetime; the request is pointing into the string
// buffer, so we need to make sure the buffer lasts as long as the Request
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    // an associated function (aka static method) to create a Request form a byte array
    fn from_byte_array(buf: &[u8]) -> Result<Self, ParseError> {
        unimplemented!();
    }
}

// this allows us to convert byte slice to Request struct
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        // extract string slices of the sections of interest
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // confirm protocol is expected version
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // parse out the HTTP method (see the FromString trait implementation for Method)
        let method: Method = method.parse()?;

        // parse out the query string
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[(i+1)..]));
            path = &path[..i];
        }

        // return the completely parsed request
        Ok(Self { path, query_string, method, })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[(i+1)..]))
        }
    }
    None
}

// set of possible errors we could encounter when trying to parse a request string
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol, // we'll only support HTTP/1.1
    InvalidMethod, // if we get a request that has an invalid HTTP verb
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
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
