#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

#[macro_use]
extern crate url;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.1.0";


#[derive(Debug, PartialEq)]
pub enum AGetResponse {
    /// OK
    OK ( String ) ,
    /// Not found
    NotFound ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum APostResponse {
    /// OK
    OK ( String ) ,
    /// Not found
    NotFound ( String ) ,
}


/// API
pub trait Api<C> {


    fn a_get(&self, context: &C) -> Box<Future<Item=AGetResponse, Error=ApiError>>;


    fn a_post(&self, arg: models::Arg, context: &C) -> Box<Future<Item=APostResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn a_get(&self) -> Box<Future<Item=AGetResponse, Error=ApiError>>;


    fn a_post(&self, arg: models::Arg) -> Box<Future<Item=APostResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn a_get(&self) -> Box<Future<Item=AGetResponse, Error=ApiError>> {
        self.api().a_get(&self.context())
    }


    fn a_post(&self, arg: models::Arg) -> Box<Future<Item=APostResponse, Error=ApiError>> {
        self.api().a_post(arg, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
