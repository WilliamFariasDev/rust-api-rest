use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use hyper::{Client, Error, Method, StatusCode};
use tokio;

// Just a simple type alias
type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn teste_url(url: hyper::Uri) -> hyper::Body {
    let client = Client::new();
    let work = client
        .get(url)
        .map(|res| {
            println!("Response: {}", res.status());
            println!("Paassou aqui:");
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });

    tokio::spawn(work);

    Body::from("rr")
}

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            let uri = "http://httpbin.org/ip".parse().unwrap();

            let res = teste_url(uri);

            *response.body_mut() = res;
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn main() {
    // rt::run(rt::lazy(|| {
    //     let client = Client::new();

    //     let uri = "http://httpbin.org/ip".parse().unwrap();

    //     client
    //         .get(uri)
    //         .map(|res| {
    //             println!("Response: {}", res.status());
    //         })
    //         .map_err(|err| {
    //             println!("Error: {}", err);
    //         })
    // }));

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
