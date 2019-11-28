#!/bin/bash

cargo package && \
cargo publish && \
./build_js.sh && \
cd ./mjs && \
npm publish
