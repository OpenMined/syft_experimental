use crate::worker::{get_config, Configurable};
use node::config_server::Config;
pub use node::{
    CapabilityReply, ConnectReply, ConnectRequest, NodeRequest, RegisterCapabilityRequest,
};
use tonic::{Request, Response, Status};

pub use node::config_client::ConfigClient;
pub use node::config_server::ConfigServer;

pub mod node {
    tonic::include_proto!("node");
}

#[derive(Debug, Default)]
pub struct ConfigService {
    node_id: i32,
}

impl ConfigService {}

#[tonic::async_trait]
impl Config for ConfigService {
    async fn capabilities(
        &self,
        request: Request<NodeRequest>,
    ) -> Result<Response<CapabilityReply>, Status> {
        println!("Got a request: {:?}", request);

        let config = get_config().clone();

        let reply = CapabilityReply {
            capability: config.lock().unwrap().capability_map.to_vec(),
        };

        Ok(Response::new(reply))
    }

    async fn register_capability(
        &self,
        request: Request<RegisterCapabilityRequest>,
    ) -> Result<Response<CapabilityReply>, Status> {
        println!("Got a request: {:?}", request);

        let config = get_config().clone();
        config
            .lock()
            .unwrap()
            .add_capability(request.into_inner().capability_name);

        let reply = CapabilityReply {
            capability: config.lock().unwrap().capability_map.to_vec(),
        };

        Ok(Response::new(reply))
    }

    async fn connect_peer(
        &self,
        request: Request<ConnectRequest>,
    ) -> Result<Response<ConnectReply>, Status> {
        println!("Got a request: {:?}", request);

        let config = get_config().clone();

        config
            .lock()
            .unwrap()
            .add_peer(request.into_inner().client_node_id);

        let reply = ConnectReply {
            node_id: config.lock().unwrap().get_node_id(),
        };

        Ok(Response::new(reply))
    }
}
