// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_even);
    register_demo!(runner, demo_integer_odd);

    register_bench!(runner, benchmark_integer_even);
    register_bench!(runner, benchmark_integer_odd);
}

fn demo_integer_even(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        if n.even() {
            println!("{n} is even");
        } else {
            println!("{n} is not even");
        }
    }
}

fn demo_integer_odd(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        if n.even() {
            println!("{n} is odd");
        } else {
            println!("{n} is not odd");
        }
    }
}

fn benchmark_integer_even(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.even()",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.even()))],
    );
}

fn benchmark_integer_odd(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.odd()",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.odd()))],
    );
}
