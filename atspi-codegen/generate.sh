#!/bin/bash

cd atspi-codegen
cargo build
cd ..
./atspi-codegen/target/debug/identify > src/identify.rs
