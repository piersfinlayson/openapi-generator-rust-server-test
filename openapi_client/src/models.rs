#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


/// An arg
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum Arg { 
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
}

impl ::std::fmt::Display for Arg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            Arg::_0 => write!(f, "{}", "0"),
            Arg::_1 => write!(f, "{}", "1"),
            Arg::_2 => write!(f, "{}", "2"),
        }
    }
}

impl ::std::str::FromStr for Arg {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Arg::_0),
            "1" => Ok(Arg::_1),
            "2" => Ok(Arg::_2),
            _ => Err(()),
        }
    }
}

/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Error(String);

impl ::std::convert::From<String> for Error {
    fn from(x: String) -> Self {
        Error(x)
    }
}

impl ::std::convert::From<Error> for String {
    fn from(x: Error) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Error {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// OK
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Ok(String);

impl ::std::convert::From<String> for Ok {
    fn from(x: String) -> Self {
        Ok(x)
    }
}

impl ::std::convert::From<Ok> for String {
    fn from(x: Ok) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Ok {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Ok {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

