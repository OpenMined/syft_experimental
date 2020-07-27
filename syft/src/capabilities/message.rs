use crate::worker::get_config;
use message::message_server::Message;
use tonic::{Request, Response, Status};

pub use message::message_client::MessageClient;
pub use message::message_server::MessageServer;
pub use message::{syft_message::Id, SyftMessage};
pub use std::collections::HashMap;

pub mod message {
    tonic::include_proto!("message");
}

#[derive(Debug, Default)]
pub struct MyMessage {}

#[tonic::async_trait]
impl Message for MyMessage {
    async fn send_message(
        &self,
        request: Request<SyftMessage>,
    ) -> Result<Response<SyftMessage>, Status> {
        // dispatch message to capability
        let config = get_config().clone();
        let message = request.into_inner();
        let capability_key = message.capability.to_owned();

        if let Some(callback) = config.lock().unwrap().capability_map.get(&capability_key) {
            let result = callback.callable.execute(message);

            match result {
                Ok(message) => Ok(Response::new(message)),
                Err(err) => Err(tonic::Status::new(
                    tonic::Code::NotFound,
                    format!(
                        "Unable to dispatch to capability {}. Error: {}",
                        &capability_key, err
                    ),
                )),
            }
        } else {
            Err(tonic::Status::new(
                tonic::Code::NotFound,
                format!("Unable to dispatch to capability: {}", &capability_key),
            ))
        }
    }
}
