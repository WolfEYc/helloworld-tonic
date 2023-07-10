use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloResponse, HelloRequest, ByeByeRequest, ByeByeResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld"); //must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>)
     -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let hello_req = request.into_inner();
        let res_msg = format!("Hello {}!", hello_req.name);

        let reply = hello_world::HelloResponse {
            message: res_msg.into() // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
    
    async fn say_bye(&self, request: Request<ByeByeRequest>)
     -> Result<Response<ByeByeResponse>, Status> {
        println!("Got a request: {:?}", request);

        let bye_req = request.into_inner();


        let res_msg = match bye_req.satisfaction_level {
            1 => "We're sorry to hear that",
            x if x < 5 => "What could we do better",
            _ => "Glad to hear you enjoyed the service"
        };

        let reply = hello_world::ByeByeResponse {
            bye_bye_msg: res_msg.into() // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter_service = 
    GreeterServer::new(MyGreeter::default());

    Server::builder()
        .add_service(greeter_service)
        .serve(addr)
        .await?;

    Ok(())
}