#!/bin/bash

# Windows
cross build --release --target x86_64-pc-windows-gnu && \
mkdir -p ./mjs/mergereq-win64 && \
cp ./target/x86_64-pc-windows-gnu/release/mergereq.exe ./mjs/mergereq-win64/ && \
echo "Binary for Windows (x64) created" && \

# Linux
cargo build --release && \
mkdir -p ./mjs/mergereq-linux64 && \
cp ./target/release/mergereq ./mjs/mergereq-linux64/ && \
echo "Binary for Linux (x64) created" && \

# OSX
# TODO


# README
cp README.md ./mjs/ && \
echo "README file copied" && \

# LICENSE
cp LICENSE ./mjs/ && \
echo "LICENSE file copied" && \

# SHASUM256
cd mjs && \
rm -f SHASUM256.txt && \
sha256sum mergereq-win64/mergereq.exe >> SHASUM256.txt && \
sha256sum mergereq-linux64/mergereq >> SHASUM256.txt && \
echo "SHASUM256.txt created"
