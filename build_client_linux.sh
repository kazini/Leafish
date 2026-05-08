#!/bin/bash

# cargo build --release --target x86_64-unknown-linux-gnu || { echo "Error building Linux client"; exit 1; }
cargo build --release || { echo "Error building Linux client"; exit 1; }

cp ./target/release/leafish ./target/leafish_x86_64_linux || { echo "Error copying Linux client"; exit 1; }

echo "Successfully built client"