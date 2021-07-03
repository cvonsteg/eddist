#!/bin/bash

# build
cargo build --release

# Add to binaries
ln -s $PWD/target/release/eddist /usr/local/bin/eddist