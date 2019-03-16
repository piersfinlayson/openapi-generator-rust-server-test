//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use openapi_client::{Api, ApiError,
                      AGetResponse,
                      APostResponse
};
use openapi_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{


    fn a_get(&self, context: &C) -> Box<Future<Item=AGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("a_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn a_post(&self, arg: models::models::models::Arg, context: &C) -> Box<Future<Item=APostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("a_post({:?}) - X-Span-ID: {:?}", arg, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
