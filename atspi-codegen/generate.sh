#!/bin/bash

cd atspi-codegen
cargo build && \
cd .. && \
./atspi-codegen/target/debug/gen_identify -f
