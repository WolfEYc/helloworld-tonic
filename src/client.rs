use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest, ByeByeRequest};


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = 
    GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
    HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("BYE RESPONSE={:?}", response);

    let request = tonic::Request::new(
    ByeByeRequest {
        satisfaction_level: 5,
    });

    let response = client.say_bye(request).await?;

    println!("BYE RESPONSE={:?}", response);

    Ok(())
}