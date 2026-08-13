// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_dedekind_sum);
    register_bench!(runner, benchmark_rational_dedekind_sum);
}

fn demo_rational_dedekind_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for (h, k) in integer_pair_gen().get(gm, config).take(limit) {
        println!(
            "dedekind_sum({}, {}) = {}",
            h,
            k,
            Rational::dedekind_sum(&h, &k)
        );
    }
}

fn benchmark_rational_dedekind_sum(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational::dedekind_sum(&Integer, &Integer)",
        BenchmarkType::Single,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("h", "k"),
        &mut [("Malachite", &mut |(h, k)| {
            no_out!(Rational::dedekind_sum(&h, &k));
        })],
    );
}
