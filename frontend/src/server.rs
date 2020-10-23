// use tonic::{transport::Server, Request, Response, Status};

// use hello_world::greeter_server::{Greeter, GreeterServer};
// use hello_world::{HelloReply, HelloRequest};

// use warp::{Filter, Reply};

// pub mod hello_world {
//     tonic::include_proto!("helloworld");
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "127.0.0.1:8000".parse()?;
//     let greeter = MyGreeter::default();

//     let grpc_header = warp::header::exact_ignore_case("content-type", "application/grpc");


//     let routes = warp::any()
//         .and(grpc_header)
//         .and(warp::body::bytes())
//         .map(|body| greeter);

//     warp::serve(routes).run(addr).await;

//     // Server::builder()
//     //     .add_service(GreeterServer::new(greeter))
//     //     .serve(addr)
//     //     .await?;

//     Ok(())
// }