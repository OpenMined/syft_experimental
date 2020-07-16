use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("protoc -I=../../protos --python_out=../../../src/syft/protos ../../protos/message.proto")
            .output()
            .expect("failed to execute process")
    };

    println!("out: {:?}", output);

    Ok(())
}
