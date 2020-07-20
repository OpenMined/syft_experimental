use crate::capabilities::helloworld::{GreeterServer, MyGreeter};
use crate::capabilities::node::{ConfigServer, ConfigService};
use crate::worker::config::{get_config, Configurable};
use tonic::transport::Server;

// create a tonic gRPC server
async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let port: u32 = 50051;
    let addr = format!("[::1]:{}", port).parse()?;
    let greeter = MyGreeter::default();
    let config = ConfigService::default();

    println!("Worker listening on {}", addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(ConfigServer::new(config))
        .serve(addr)
        .await?;
    Ok(())
}

// launch gRPC server inside existing global tokio runtime
pub fn start_on_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let lock = get_config().try_lock();
    match lock {
        Ok(mut config) => {
            let rt = config.get_runtime();
            let handle = rt.handle().clone();
            // block here so that the async code gets executed
            return handle.block_on(start_server());
        }
        Err(err) => {
            let message = format!("Failed to acquire lock: {}", err);
            return Err(message)?;
        }
    }
}

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    start_server().await
}
