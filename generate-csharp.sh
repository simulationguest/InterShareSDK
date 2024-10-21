#!/bin/bash

pushd src/intershare_sdk_ffi
    cargo build src/intershare_sdk_ffi --lib --release --feautures sync --target x86_64-pc-windows-msvc
    cargo build src/intershare_sdk_ffi --lib --release --feautures sync --target aarch64-pc-windows-msvc
popd

uniffi-bindgen-cs target/x86_64-pc-windows-msvc/release/libintershare_sdk_ffi.so --library --out-dir="bindings/csharp/InterShareSDK/"
