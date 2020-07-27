use crate::capabilities::message::SyftMessage;
use crate::worker::{get_config, Configurable};

/// connect to another node
pub fn connect(url: String) -> String {
    get_config().clone().lock().unwrap().connect_peer(url)
}

/// connect to another node and say hello
pub fn say_hello(url: String, name: String) -> String {
    get_config().clone().lock().unwrap().say_hello(url, name)
}

/// connect to another node and request capabilities
pub fn request_capabilities(url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    get_config()
        .clone()
        .lock()
        .unwrap()
        .request_capabilities(url)
}

/// connect to another node and say hello
pub fn run_class_method_message(
    url: String,
    message: SyftMessage,
) -> Result<SyftMessage, Box<dyn std::error::Error>> {
    let config = get_config();

    let result = config
        .clone()
        .lock()
        .unwrap()
        .run_class_method_message(url, message);

    match result {
        Ok(message) => Ok(message),
        Err(_err) => {
            println!("Got an error {:?}", _err);
            Err(format!("Got an error: {:?}", _err))?
        }
    }
}
