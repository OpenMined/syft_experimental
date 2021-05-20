use crate::capabilities::helloworld::{GreeterClient, HelloRequest};
use crate::capabilities::message::{MessageClient, SyftMessage};
use crate::capabilities::node::{ConfigClient, ConnectRequest, NodeRequest};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;
use uuid::Uuid;

lazy_static! {
    pub static ref WORKER_CONFIG: Arc<Mutex<WorkerConfig>> =
        Arc::new(Mutex::new(WorkerConfig::new()));
}

pub fn get_config() -> &'static WORKER_CONFIG {
    &WORKER_CONFIG
}

pub fn get_runtime_handle() -> Result<Handle, Box<dyn std::error::Error>> {
    let lock = get_config().lock();
    match lock {
        Ok(mut config) => {
            let rt = config.get_runtime();
            let handle = rt.handle().clone();
            // drop the lock so that others can access the WORKER_CONFIG
            drop(config);
            Ok(handle)
        }
        Err(err) => {
            let message = format!("Failed to acquire lock: {}", err);
            return Err(message)?;
        }
    }
}

pub trait Configurable {
    fn new() -> WorkerConfig;
    //fn start_server(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn add_peer(&mut self, peer_id: String);
    fn get_peers(&self) -> &Vec<String>;
    fn add_capability(&mut self, capability: String, callback: Callback);
    fn get_node_id(&self) -> String;
    fn get_runtime(&mut self) -> &tokio::runtime::Runtime;
    fn get_client(
        &mut self,
        url: String,
    ) -> Result<ConfigClient<tonic::transport::channel::Channel>, tonic::transport::Error>;
    fn connect_peer(&mut self, url: String) -> String;
    fn say_hello(&mut self, url: String, name: String) -> String;
    fn run_class_method_message(
        &mut self,
        url: String,
        message: SyftMessage,
    ) -> Result<SyftMessage, Box<dyn std::error::Error>>;
    fn request_capabilities(
        &mut self,
        url: String,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct Callback {
    pub callable: Box<dyn Callable>,
}

pub trait Callable: Send {
    fn execute(&self, message: SyftMessage) -> Result<SyftMessage, Box<dyn std::error::Error>>;
}

use core::fmt::Debug;
impl Debug for dyn Callable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Callable: {}", std::any::type_name::<Self>())
    }
}

#[derive(Debug)]
pub struct WorkerConfig {
    runtime: tokio::runtime::Runtime,
    node_id: String,
    connected_peers: Vec<String>, // should be Uuid, need to figure out how to represent Uuid in proto
    pub capability_map: HashMap<String, Callback>,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        WorkerConfig {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .on_thread_start(|| {
                    println!("Tokio thread started");
                })
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            node_id: Uuid::new_v4().to_simple().to_string(),
            connected_peers: vec![],
            capability_map: HashMap::new(),
        }
    }
}

impl Configurable for WorkerConfig {
    fn new() -> WorkerConfig {
        Default::default() // see impl Default for WorkerConfig
    }

    fn add_peer(&mut self, peer_id: String) {
        self.connected_peers.push(peer_id);
    }

    fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

    fn add_capability(&mut self, capability: String, callback: Callback) {
        self.capability_map.insert(capability, callback);
    }

    fn get_peers(&self) -> &Vec<String> {
        &self.connected_peers
    }

    fn get_runtime(&mut self) -> &tokio::runtime::Runtime {
        &self.runtime
    }

    fn get_client(
        &mut self,
        url: String,
    ) -> Result<ConfigClient<tonic::transport::channel::Channel>, tonic::transport::Error> {
        match self.runtime.block_on(ConfigClient::connect(url)) {
            Ok(_client) => Ok(_client),
            Err(_err) => Err(_err),
        }
    }

    fn connect_peer(&mut self, url: String) -> String {
        let request = tonic::Request::new(ConnectRequest {
            client_node_id: "test".into(),
        });
        match self.runtime.block_on(ConfigClient::connect(url)) {
            Ok(mut _client) => match self.runtime.block_on(_client.connect_peer(request)) {
                Ok(_response) => _response.get_ref().node_id.to_string(),
                Err(_err) => String::from("0"),
            },
            Err(_err) => String::from("0"),
        }
    }

    fn request_capabilities(
        &mut self,
        url: String,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let node_id = self.get_node_id();
        let request = tonic::Request::new(NodeRequest { node_id: node_id });
        match self.runtime.block_on(ConfigClient::connect(url)) {
            Ok(mut _client) => match self.runtime.block_on(_client.capabilities(request)) {
                Ok(_response) => {
                    let caps = &_response.get_ref().capability;
                    println!("{}", format!("Capabilities returned: {:?}", caps));
                    Ok(caps.clone())
                }
                Err(_err) => Err(format!("No Capabilities Returned: {}", _err))?,
            },
            Err(_err) => Err(format!("No Capabilities Returned: {}", _err))?,
        }
    }

    fn say_hello(&mut self, url: String, name: String) -> String {
        let request = tonic::Request::new(HelloRequest { name: name });

        match self.runtime.block_on(GreeterClient::connect(url)) {
            Ok(mut _client) => match self.runtime.block_on(_client.say_hello(request)) {
                Ok(_response) => {
                    println!("got response {:?}", _response);
                    _response.get_ref().message.to_string()
                }
                Err(_err) => {
                    println!("failed to get response from client: {:?}", _err);
                    String::from("say hello Failed")
                }
            },
            Err(_err) => {
                println!("failed to connect to client: {:?}", _err);
                String::from("say hello Failed")
            }
        }
    }

    fn run_class_method_message(
        &mut self,
        url: String,
        message: SyftMessage,
    ) -> Result<SyftMessage, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(message);

        match self.runtime.block_on(MessageClient::connect(url)) {
            Ok(mut _client) => match self.runtime.block_on(_client.send_message(request)) {
                Ok(_response) => {
                    let message = _response.into_inner();
                    Ok(message)
                }
                Err(_err) => {
                    println!("failed to get response from client run class: {:?}", _err);
                    Err(format!("send_message Failed: {:?}", _err))?
                }
            },
            Err(_err) => {
                println!("failed to connect to client: {:?}", _err);
                Err(format!("failed to connect to client Failed: {:?}", _err))?
            }
        }
    }
}
