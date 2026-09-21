// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_u64_polynomial_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::u64_polynomial_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u64_polynomial_cmp);

    register_bench!(runner, benchmark_u64_polynomial_cmp);
}

fn demo_u64_polynomial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in u64_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("({}).cmp(&{}) = {:?}", p, q, p.cmp(&q));
    }
}

// `cmp`'s result is what is being timed, so the benchmark discards it on purpose.
#[allow(unused_must_use)]
fn benchmark_u64_polynomial_cmp(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "U64Polynomial.cmp(&U64Polynomial)",
        BenchmarkType::Single,
        u64_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_u64_polynomial_max_bit_bucketer("p", "q"),
        &mut [("Malachite", &mut |(p, q)| no_out!(p.cmp(&q)))],
    );
}
