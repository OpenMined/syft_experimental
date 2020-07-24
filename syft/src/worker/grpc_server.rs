use crate::capabilities::helloworld::{GreeterServer, MyGreeter};
use crate::capabilities::message::{MessageServer, MyMessage};
use crate::capabilities::node::{ConfigServer, ConfigService};
use crate::worker::config::{get_config, get_runtime_handle, Callback, Configurable};
use tonic::transport::Server;

// create a tonic gRPC server
async fn start_server(port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;
    let greeter = MyGreeter::default();
    let message = MyMessage::default();
    let config = ConfigService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(MessageServer::new(message))
        .add_service(ConfigServer::new(config))
        .serve(addr)
        .await?;
    Ok(())
}

// launch gRPC server inside existing global tokio runtime
pub fn start_on_runtime(port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("[::1]:{}", port).parse()?;
    println!("Starting node on {}", addr);
    let status = get_runtime_handle()?.block_on(start_server(port));
    match status {
        Ok(()) => Ok(()),
        Err(err) => {
            if format!("{}", err).contains("Address already in use") {
                println!("Port {} taken", port);
                return start_on_runtime(port + 1);
            }
            Err(err)
        }
    }
}

pub fn add_capability(
    capability_name: String,
    callback: Callback,
) -> Result<(), Box<dyn std::error::Error>> {
    let lock = get_config().lock();
    match lock {
        Ok(mut config) => {
            config.add_capability(capability_name, callback);
            drop(config);
            Ok(())
        }
        Err(err) => {
            let message = format!("Failed to acquire lock: {}", err);
            return Err(message)?;
        }
    }
}

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let port: u32 = 50051;
    start_server(port).await
}
