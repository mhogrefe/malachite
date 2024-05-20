#!/bin/bash
P=$PWD &&
cd ../../doc_runner &&
cargo run --release -- $P "$@"
