use crate::capabilities::node::{ConfigClient, ConnectRequest};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

lazy_static! {
    pub static ref WORKER_CONFIG: Arc<Mutex<WorkerConfig>> =
        Arc::new(Mutex::new(WorkerConfig::new()));
}

pub fn get_config() -> &'static WORKER_CONFIG {
    &WORKER_CONFIG
}

pub trait Configurable {
    fn new() -> WorkerConfig;
    //fn start_server(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn add_peer(&mut self, peer_id: String);
    fn get_peers(&self) -> &Vec<String>;
    fn add_capability(&mut self, capability: String);
    fn get_node_id(&self) -> String;
    fn get_runtime(&mut self) -> &tokio::runtime::Runtime;
    fn get_client(
        &mut self,
        url: String,
    ) -> Result<ConfigClient<tonic::transport::channel::Channel>, tonic::transport::Error>;
    fn connect_peer(&mut self, url: String) -> String;
}

#[derive(Debug)]
pub struct WorkerConfig {
    run_time: tokio::runtime::Runtime,
    node_id: String,
    connected_peers: Vec<String>, // should be Uuid, need to figure out how to represent Uuid in proto
    pub capability_map: Vec<String>,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        WorkerConfig {
            run_time: tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .unwrap(),
            node_id: Uuid::new_v4().to_simple().to_string(),
            connected_peers: vec![],
            capability_map: vec![],
        }
    }
}

impl Configurable for WorkerConfig {
    fn new() -> WorkerConfig {
        WorkerConfig {
            run_time: tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .unwrap(),
            node_id: Uuid::new_v4().to_simple().to_string(),
            connected_peers: vec![],
            capability_map: vec![],
        }
    }

    fn add_peer(&mut self, peer_id: String) {
        self.connected_peers.push(peer_id);
    }

    fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

    fn add_capability(&mut self, capability: String) {
        self.capability_map.push(capability);
    }

    fn get_peers(&self) -> &Vec<String> {
        &self.connected_peers
    }

    fn get_runtime(&mut self) -> &tokio::runtime::Runtime {
        &self.run_time
    }

    fn get_client(
        &mut self,
        url: String,
    ) -> Result<ConfigClient<tonic::transport::channel::Channel>, tonic::transport::Error> {
        match self.run_time.block_on(ConfigClient::connect(url)) {
            Ok(_client) => Ok(_client),
            Err(_err) => Err(_err),
        }
    }

    fn connect_peer(&mut self, url: String) -> String {
        let request = tonic::Request::new(ConnectRequest {
            client_node_id: "test".into(),
        });
        match self.run_time.block_on(ConfigClient::connect(url)) {
            Ok(mut _client) => match self.run_time.block_on(_client.connect_peer(request)) {
                Ok(_response) => _response.get_ref().node_id.to_string(),
                Err(_err) => String::from("0"),
            },
            Err(_err) => String::from("0"),
        }
    }
}
