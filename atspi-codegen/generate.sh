#!/bin/bash

cargo build
cd ..
./atspi-codegen/target/debug/identify > src/identify.rs
