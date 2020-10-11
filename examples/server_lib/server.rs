//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;
use swagger;
use swagger::{Has, XSpanIdString};

use openapi_client::{Api, ApiError,
                      CountGetResponse,
                      DogGetResponse
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

    /// Returns the number of times this endpoint has been called.
    fn count_get(&self, context: &C) -> Box<Future<Item=CountGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("count_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Returns the string \"woof\".
    fn dog_get(&self, context: &C) -> Box<Future<Item=DogGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("dog_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(DogGetResponse("woof"))
    }

}
