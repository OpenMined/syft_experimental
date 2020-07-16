use crate::capabilities::helloworld::{GreeterServer, MyGreeter};
use crate::capabilities::node::{ConfigServer, ConfigService};
use tonic::transport::Server;

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // let (tx, rx) = mpsc::sync_channel(1024);
    // let shared_rx = Arc::new(Mutex::new(rx));

    // let mut workers = vec![];

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let config = ConfigService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(ConfigServer::new(config))
        .serve(addr)
        .await?;

    Ok(())
}
