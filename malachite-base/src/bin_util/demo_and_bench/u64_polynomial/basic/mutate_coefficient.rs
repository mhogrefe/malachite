// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_1_u64_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::u64_polynomial_unsigned_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u64_polynomial_mutate_coefficient);
    register_bench!(runner, benchmark_u64_polynomial_mutate_coefficient);
}

fn demo_u64_polynomial_mutate_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, i) in u64_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old = p.to_string();
        p.mutate_coefficient(i, |c| *c += 1);
        println!("({old}).mutate_coefficient({i}, |c| *c += 1) = {p}");
    }
}

fn benchmark_u64_polynomial_mutate_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "U64Polynomial.mutate_coefficient(u64, FnOnce)",
        BenchmarkType::Single,
        u64_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_u64_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(mut p, i)| {
            no_out!(p.mutate_coefficient(i, |c| *c += 1));
        })],
    );
}
