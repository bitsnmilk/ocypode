extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};

struct SummaryConfig(u32, char);

struct Config {
    author: String,
    title: String,
    root: String,
    date_formatter: fn(String) -> String,
    summary: SummaryConfig,
    ext: String,
    cache: u32,
}

struct Site<'a> {
    config: &'a Config,
}

struct OcypodeServer<'a> {
    config: &'a Config,
    site: Site<'a>,
}

impl<'a> OcypodeServer<'a> {
    fn hello_world(_req: Request<Body>) -> Response<Body> {
        Response::new(Body::from("Hello World"))
    }

    fn start(self, port: u16) {
        let addr = ([127, 0, 0, 1], 3000).into();
        let new_svc = || service_fn_ok(OcypodeServer::hello_world);

        let server = Server::bind(&addr)
            .serve(new_svc)
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

    let ocypode = OcypodeServer {
        config: &config,
        site: Site { config: &config },
    };

    ocypode.start(3000);
}
