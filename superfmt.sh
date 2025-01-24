#!/bin/bash
cargo fmt --all > /dev/null 2>&1 &&
P=$PWD &&
cd ../../format_comments &&
cargo run --release -- $P
