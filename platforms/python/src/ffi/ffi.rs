use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyUnicode};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use std::thread;
use syft::capabilities::message::SyftMessage;
use syft::worker::{add_capability, start_on_runtime, Callable, Callback};

// the module will be syft but with a mixed python project it becomes syft.syft
// so this needs to be re-exported from a __init__.py file with: from .syft import *
#[pymodule]
fn syft(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(message))?;
    m.add_wrapped(wrap_pymodule!(node))?;
    Ok(())
}

#[pymodule]
fn message(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(run_class_method_message))?;
    m.add_wrapped(wrap_pyfunction!(say_hello))?;
    Ok(())
}

#[pymodule]
fn node(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(start))?;
    m.add_wrapped(wrap_pyfunction!(register))?;
    m.add_wrapped(wrap_pyfunction!(connect))?;
    m.add_wrapped(wrap_pyfunction!(request_capabilities))?;
    Ok(())
}

#[pyfunction]
fn start() -> PyResult<()> {
    let port: u32 = 50051;
    thread::spawn(move || {
        let result = start_on_runtime(port);
        match result {
            Ok(_) => println!("gRPC Server thread finished"),
            Err(err) => println!("gRPC Server thread failed with error. {}", err),
        }
    });

    Ok(())
}

struct PyCallback(PyObject);

impl Callable for PyCallback {
    fn execute(&self, message: SyftMessage) -> Result<SyftMessage, Box<dyn std::error::Error>> {
        let mut message_bytes = vec![];
        to_bytes(&message, &mut message_bytes).expect("Rust Failed to encode message");

        let gil = Python::acquire_gil();
        let py = gil.python();
        let py_bytes = PyBytes::new(py, message_bytes.as_slice());

        let py_result: PyResult<PyObject> = self.0.call1(py, (py_bytes,));

        // lets get the result of the function back into py_bytes
        let py_bytes: &PyBytes;
        let response: SyftMessage;

        if let Ok(result) = py_result {
            if let Ok(bytes) = result.extract(py) {
                py_bytes = bytes;
                response = from_bytes(py_bytes.as_bytes()).expect("Rust Failed to decode message");
                return Ok(response);
            }
        };

        Err(format!(
            "Failed to execute capability: {} in python",
            message.capability
        ))?
    }
}

#[allow(dead_code)]
#[pyfunction]
fn register(py_capability_name: &PyUnicode, py_callback: &PyAny) -> PyResult<()> {
    // bring the function over to the dark side
    let name: String = py_capability_name.extract()?;
    let callback: PyObject = py_callback.into();

    let cb1 = Box::new(PyCallback(callback));
    let cb = Callback { callable: cb1 };

    let result = add_capability(name.clone(), cb);
    match result {
        Ok(_) => println!("Capability registered: {}", name),
        Err(err) => println!("Failed to register capability {}. {}", name, err),
    }

    Ok(())
}

#[pyfunction]
pub fn connect(target_addr: &PyUnicode) -> PyResult<()> {
    let addr: String = target_addr.extract()?;
    syft::client::connect(addr);
    Ok(())
}

#[pyfunction]
pub fn request_capabilities(target_addr: &PyUnicode) -> PyResult<Vec<String>> {
    let addr: String = target_addr.extract()?;
    let response = syft::client::request_capabilities(addr);
    match response {
        Ok(caps) => Ok(caps),
        Err(_err) => Err(PyErr::new::<pyo3::exceptions::Exception, _>(
            "unable to run request_capabilities",
        )),
    }
}

#[pyfunction]
pub fn say_hello(target_addr: &PyUnicode, name: &PyUnicode) -> PyResult<String> {
    let addr: String = target_addr.extract()?;
    let name: String = name.extract()?;
    let result = syft::client::say_hello(addr, name).into();
    Ok(result)
}

#[pyfunction]
fn run_class_method_message(
    target_addr: &PyUnicode,
    py_bytes: &PyBytes,
) -> PyResult<std::vec::Vec<u8>> {
    // deserialize
    let request: SyftMessage;
    request = from_bytes(py_bytes.as_bytes()).expect("Rust Failed to decode message");

    let addr: String = target_addr.extract()?;
    let response = syft::client::run_class_method_message(addr, request);

    // serialize
    match response {
        Ok(message) => {
            let mut response_bytes = vec![];
            to_bytes(&message, &mut response_bytes).expect("Rust Failed to encode message");
            return Ok(response_bytes);
        }
        Err(_err) => Err(PyErr::new::<pyo3::exceptions::Exception, _>(
            "unable to run run_class_method_message",
        )),
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
