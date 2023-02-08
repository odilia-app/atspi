#!/bin/bash

cd atspi-codegen
cargo build
cd ..
./atspi-codegen/target/debug/identify > src/identify.rs
./atspi-codegen/target/debug/event_names > src/events/names.rs
