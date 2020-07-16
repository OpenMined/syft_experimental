use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyUnicode};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use syft::message::SyftMessage;

#[pymodule]
fn syft(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(message))?;
    Ok(())
}

#[pymodule]
fn message(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(run_class_method_message))?;
    Ok(())
}

#[pyfunction]
fn run_class_method_message(
    target_addr: &PyUnicode,
    capability_name: &PyUnicode,
    py_bytes: &PyBytes,
) -> PyResult<std::vec::Vec<u8>> {
    println!(
        "Rust got Python Request {:?} {:?} {:?}",
        target_addr, capability_name, py_bytes
    );

    // deserialize
    let request: SyftMessage;
    request = from_bytes(py_bytes.as_bytes()).expect("Rust Failed to decode message");
    println!("Rust deserialized message {:?}", request);

    // serialize
    let mut response_bytes = vec![];
    to_bytes(&request, &mut response_bytes).expect("Rust Failed to encode message");

    println!("Rust sending back message as bytes {:?}", request);

    return Ok(response_bytes);
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
pub fn from_bytes<M: prost::Message + std::default::Default>(
    buf: &[u8],
) -> Result<M, prost::DecodeError> {
    let msg = prost::Message::decode(buf);
    return msg;
}
