/*
 * Exemplar-Rust
 *
 * An exemplar rust repo with an openapi spec
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: decline@umass.edu
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct DefaultApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DefaultApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient {
            configuration,
        }
    }
}

pub trait DefaultApi {
    fn count_get(&self, ) -> Box<dyn Future<Item = i32, Error = Error<serde_json::Value>>>;
    fn dog_get(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>DefaultApi for DefaultApiClient<C> {
    fn count_get(&self, ) -> Box<dyn Future<Item = i32, Error = Error<serde_json::Value>>> {
        let req = __internal_request::Request::new(hyper::Method::Get, "/count".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn dog_get(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let req = __internal_request::Request::new(hyper::Method::Get, "/dog".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}