extern crate futures;
extern crate hyper;

use futures::future;
use futures::future::Future;
use futures::future::IntoFuture;
use hyper::service::{NewService, Service};
use hyper::{Body, Request, Response, Server};

#[derive(Clone)]
struct SummaryConfig(u32, char);

#[derive(Clone)]
struct Config {
    author: String,
    title: String,
    root: String,
    date_formatter: fn(String) -> String,
    summary: SummaryConfig,
    ext: String,
    cache: u32,
}

#[derive(Clone)]
struct Site {
    config: Config,
}

impl Site {
    fn build_response(&self) -> Result<String, ()> {
        Ok(String::from("Hello world!"))
    }
}

#[derive(Clone)]
struct App {
    config: Config,
    site: Site,
}

#[derive(Clone)]
struct OcypodeServer {
    app: App,
}

impl Service for OcypodeServer {
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

impl IntoFuture for OcypodeServer {
    type Future = future::FutureResult<Self::Item, Self::Error>;
    type Item = Self;
    type Error = hyper::Error;

    fn into_future(self) -> Self::Future {
        future::ok(self)
    }
}

impl NewService for OcypodeServer {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = OcypodeServer;

    type Future = future::FutureResult<Self::Service, Self::Error>;
    type InitError = Self::Error;

    fn new_service(&self) -> Self::Future {
        self.clone().into_future()
    }
}

impl OcypodeServer {
    fn start(self, port: u16) {
        let addr = ([127, 0, 0, 1], port).into();

        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}

fn main() {
    let config = Config {
        author: String::from("Self"),
        title: String::from("Title"),
        root: String::from("/"),
        date_formatter: |s| s,
        summary: SummaryConfig(150, '~'),
        ext: String::from("md"),
        cache: 1200,
    };

    let app = App {
        config: config.clone(),
        site: Site {
            config: config.clone(),
        },
    };

    let ocypode = OcypodeServer { app: app };

    ocypode.start(3000);
}
