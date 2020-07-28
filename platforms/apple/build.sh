#!/bin/bash
cargo lipo --release
cbindgen src/lib.rs -l c > ./swift/include/syft.h
mkdir -p ./swift/libs
cp target/universal/release/libsyft.a ./swift/libs
