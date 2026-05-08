#!/bin/bash

cargo build --release --target x86_64-pc-windows-gnu || { echo "Error building Windows client"; exit 1; }

cp ./target/x86_64-pc-windows-gnu/release/leafish.exe ./target/leafish_x86_64_windows.exe || { echo "Error copying Windows client"; exit 1; }

echo "Successfully built client"