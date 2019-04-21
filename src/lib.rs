extern crate futures;
extern crate hyper;

use futures::future;
use futures::future::Future;
use futures::future::IntoFuture;
use hyper::service::{NewService, Service};
use hyper::{Body, Request, Response, Server};

pub mod app;
pub mod config;
pub mod site;

pub use app::*;
pub use config::*;
pub use site::*;

#[derive(Clone)]
pub struct Ocypode {
    app: App,
}

impl Service for Ocypode {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let mut response = Response::new(Body::empty());
        let (parts, _) = req.into_parts();
        let (path, mime) = match parts.uri.path().find(".") {
            Some(index) => parts.uri.path().split_at(index),
            None => (parts.uri.path(), ""),
        };
        let route: Vec<&str> = path.split("/").filter(|p| !p.is_empty()).collect();

        println!("hello world {:?}", route);

        Box::new(future::ok(response))
    }
}

impl IntoFuture for Ocypode {
    type Future = future::FutureResult<Self::Item, Self::Error>;
    type Item = Self;
    type Error = hyper::Error;

    fn into_future(self) -> Self::Future {
        future::ok(self)
    }
}

impl NewService for Ocypode {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = Ocypode;

    type Future = future::FutureResult<Self::Service, Self::Error>;
    type InitError = Self::Error;

    fn new_service(&self) -> Self::Future {
        self.clone().into_future()
    }
}

impl Ocypode {
    pub fn new(app: App) -> Ocypode {
        Ocypode { app }
    }

    pub fn start(self, port: u16) {
        let addr = ([127, 0, 0, 1], port).into();

        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}
