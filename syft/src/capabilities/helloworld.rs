use crate::worker::{get_config, Configurable};
use hello_world::greeter_server::Greeter;
use tonic::{Request, Response, Status};

pub use hello_world::greeter_client::GreeterClient;
pub use hello_world::greeter_server::GreeterServer;
pub use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let node_id = get_config().lock().unwrap().get_node_id();

        let reply = hello_world::HelloReply {
            message: format!("Hello {}! From Node {}", request.into_inner().name, node_id).into(),
        };

        Ok(Response::new(reply))
    }
}
