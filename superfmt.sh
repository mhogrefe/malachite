#!/bin/bash
cargo +nightly fmt --all > /dev/null 2>&1 &&
P=$PWD &&
cd ../../format_comments &&
cargo run --release -- $P
