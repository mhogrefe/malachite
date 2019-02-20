#!/bin/bash
cd malachite-test &&
cargo run --release --features 64_bit_limbs --no-default-features -- exhaustive 100000 all
