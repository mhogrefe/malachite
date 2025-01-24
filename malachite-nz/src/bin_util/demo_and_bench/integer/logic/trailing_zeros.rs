// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_nz::test_util::integer::logic::trailing_zeros::integer_trailing_zeros_alt;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_trailing_zeros);
    register_bench!(runner, benchmark_integer_trailing_zeros_algorithms);
}

fn demo_integer_trailing_zeros(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

fn benchmark_integer_trailing_zeros_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.trailing_zeros()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.trailing_zeros())),
            ("using bits explicitly", &mut |n| {
                no_out!(integer_trailing_zeros_alt(&n))
            }),
        ],
    );
}
