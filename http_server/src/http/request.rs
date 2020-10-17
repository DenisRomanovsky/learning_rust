use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::str::Utf8Error;
use std::fmt::{ Debug, Formatter, Display, Result as FmtResult};

// 'buf - lifetime of the buffer, on which Request relies.
// !!! We have to explicitly specify a lifetime for every reference that we store inside of a struct
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>, // Means that query_string may be null or String
    method: Method,
}

/*
Possible, but not Rust-idiomatic. 
impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
    }    
}
*/

// This is the Rust-way of type conversion.
// Trait implementation.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
    /*
      match str::from_utf8(buf) {
          Ok(request) => {},
          Err(_) => return Err(ParseError::InvalidEncoding)
      }
      */

      // A short cut - using ? 
      // It takes Result, unwraps and returns error or Ok result. 

      let request = str::from_utf8(buf)?;

      let(method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // Variable shadowing. request here is rewritten.
      let(mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      let(protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      
      if protocol != "HTTP/1.1" {
          return Err(ParseError::InvalidProtocol);
      }

      let method: Method = method.parse()?;
      let mut query_string = None;
     
      // If matcher (find) finds a match, it returns the index, And we handle this index,
      if let Some(i) = path.find('?'){// Regex here
        query_string = Some(&path[i+1..]);
        path = &path[..i];
      }

     Ok(Self{
         path,
         query_string,
         method
     })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..])); // +1 means +1 byte!!!
        }
    }

    None
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// Proper way to implement Rust-way errors.
impl Error for ParseError { }

//Required by Error trait

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Needed for the ? to work. As ? will try to convert Utf8Error to ParseError.
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
