use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::thread;
use syft::capabilities::message::SyftMessage;
use syft::worker::{add_capability, start_on_runtime, Callable, Callback};

#[no_mangle]
pub extern "C" fn start(c_str_iface: *const c_char, port: u32) {
    println!(
        "Rust got interface and port for binding {:?}:{:?}",
        c_str_iface, port
    );
    let c_str = unsafe { CStr::from_ptr(c_str_iface) };

    // [::] means all ipv6 interfaces like 0.0.0.0 in ipv4
    let mut default_iface = "[::]".to_owned();
    if let Ok(iface) = c_str.to_str() {
        println!("Rust extracted iface {:?}", iface);
        default_iface = iface.to_owned();
    }

    thread::spawn(move || {
        let result = start_on_runtime(default_iface.clone(), port);
        match result {
            Ok(_) => println!("gRPC Server thread finished"),
            Err(err) => println!("gRPC Server thread failed with error. {}", err),
        }
    });

    println!("Rust finished running start");
}

#[no_mangle]
pub unsafe extern "C" fn register_handler(capability_name: *const c_char, handle: CallbackHandle) {
    let c_str = unsafe { CStr::from_ptr(capability_name) };
    if let Ok(name) = c_str.to_str() {
        println!("Decoded capability name {}", name);
        let cb1 = Box::new(SwiftCallback(handle));
        let cb = Callback { callable: cb1 };

        let result = add_capability(name.to_owned(), cb);

        match result {
            Ok(_) => println!("Capability registered: {}", name),
            Err(err) => println!("Failed to register capability {}. {}", name, err),
        }
    }
}

#[repr(C)]
pub struct CallbackHandle {
    pub callback: extern "C" fn(u32, *mut c_void),
    pub return_data: *mut c_void,
}

struct SwiftCallback(CallbackHandle);

// TODO: this hasnt been checked, we are just ignoring the error
unsafe impl Send for SwiftCallback {}

impl Callable for SwiftCallback {
    fn execute(&self, message: SyftMessage) -> Result<SyftMessage, Box<dyn std::error::Error>> {
        let mut message_bytes = vec![];
        to_bytes(&message, &mut message_bytes).expect("Rust Failed to encode message");

        println!("calling swift handler capability");
        let callback = self.0.callback;
        callback(1, self.0.return_data);

        // TODO: implement proper return
        Err(format!(
            "Failed to execute capability: {} in python",
            message.capability
        ))?
    }
}

/// Encodes the message to a `Vec<u8>`.
pub fn to_bytes<M: prost::Message>(
    message: &M,
    buf: &mut Vec<u8>,
) -> Result<(), prost::EncodeError> {
    buf.reserve(message.encoded_len());
    return message.encode(buf);
}

// Decodes an message from the buffer.
pub fn from_bytes<M: prost::Message + Default>(buf: &[u8]) -> Result<M, prost::DecodeError> {
    let msg = prost::Message::decode(buf);
    return msg;
}
