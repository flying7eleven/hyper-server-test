#[macro_use]
extern crate log;

use core::fmt;
use futures::future::Future;
use futures::{future, IntoFuture};
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use std::error::Error;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum Never {}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl Error for Never {
    fn description(&self) -> &str {
        match *self {}
    }
}

struct Microservice;

impl Service for Microservice {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Box<dyn Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, request: Request<Self::ReqBody>) -> Self::Future {
        info!("Microservice received a request: {:?}", request);
        Box::new(futures::future::ok(Response::new(Body::empty())))
    }
}

impl IntoFuture for Microservice {
    type Future = future::FutureResult<Self::Item, Self::Error>;
    type Item = Self;
    type Error = Never;

    fn into_future(self) -> Self::Future {
        future::ok(self)
    }
}

fn main() {
    env_logger::init();
    let server_details = "127.0.0.1:8080";
    let address: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");
    let server = Server::bind(&address)
        .serve(|| Microservice {})
        .map_err(|e| error!("server error: {}", e));
    hyper::rt::run(server);
}
