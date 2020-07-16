use crate::worker::{get_config, Configurable};

/// connect to another node
pub fn connect(url: String) -> String {
    get_config().clone().lock().unwrap().connect_peer(url)
}
