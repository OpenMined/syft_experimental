use std::fs;

/// dynamically compiles all protos
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/message.proto")?;
    let files = fs::read_dir("../protos").unwrap();

    files
        .filter_map(Result::ok)
        .filter_map(|d| {
            d.path()
                .to_str()
                .and_then(|f| if f.ends_with(".proto") { Some(d) } else { None })
        })
        .for_each(|f| {
            println!("file: {:?}", f);
            tonic_build::compile_protos(f.path())
                .unwrap_or_else(|e| panic!("Failed to compile proto {:?}", e));
        });

    Ok(())
}
