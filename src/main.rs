pub mod http;

use env_logger::Env;
use std::net::SocketAddr;

use actix_web::{middleware, App, HttpServer};
use futures_util::future::join;
use tonic::{transport::Server, Request, Response, Status};

use hello::hello_server::{Hello, HelloServer};
use hello::{HelloRequest, HelloResponse};

use http::hello_http;

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn call(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            msg: "Ok".to_string(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let gaddr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
    let haddr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let hello_service = HelloService::default();

    let grpc = async move {
        tokio::task::spawn(
            Server::builder()
                .add_service(HelloServer::new(hello_service))
                .serve(gaddr),
        )
    };

    let http = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello_http)
    })
    .bind(haddr)?
    .run();

    println!("Listening on http://{} and http://{}", gaddr, haddr);

    let _ret = join(grpc, http).await;

    Ok(())
}
