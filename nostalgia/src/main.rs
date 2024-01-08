use std::{
    convert::Infallible,
    future::{ready, Ready},
    task::{Context, Poll},
};

use hyper::{service::Service, Request, Response};
use hyper::{body::Body};
use hyper::server;

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    Server::bind(&([127, 0, 0, 1], 1025).into())
        .serve(MyServiceFactory)
        .await
        .unwrap();
}

struct MyServiceFactory;

impl Service<&TcpStream> for MyServiceFactory {
    type Response = MyService;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;
/*
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }
*/
    fn call(&mut self, req: &TcpStream) -> Self::Future {
        println!("Accepted connection from {}", req.remote_addr());
        ready(Ok(MyService))
    }
}

struct MyService;

impl Service<Request<dyn Body>> for MyService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;
/*
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }
*/
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        println!("Handling {req:?}");
        ready(Ok(Response::builder().body("Hello World!\n".into()).unwrap()))
    }
}