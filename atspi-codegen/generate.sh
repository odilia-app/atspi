#!/bin/bash

cargo build
cd ..
./atspi-codegen/target/debug/gen_identify -f
