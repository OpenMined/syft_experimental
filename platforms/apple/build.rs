use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("sh")
        .arg("-c")
        .arg("protoc -I=../../protos --swift_out=./swift/protos ../../protos/message.proto")
        .output()
        .expect("failed to execute process");

    Ok(())
}
