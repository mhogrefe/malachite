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
use malachite_nz::integer_polynomial::ShortlexIntegerPolynomialRef;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_shortlex_integer_polynomial_cmp);

    register_bench!(runner, benchmark_shortlex_integer_polynomial_cmp);
}

fn demo_shortlex_integer_polynomial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        println!(
            "({}).cmp(&{}) = {:?}",
            p,
            q,
            ShortlexIntegerPolynomialRef(&p).cmp(&ShortlexIntegerPolynomialRef(&q))
        );
    }
}

// `cmp`'s result is what is being timed, so the benchmark discards it on purpose.
#[allow(unused_must_use)]
fn benchmark_shortlex_integer_polynomial_cmp(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ShortlexIntegerPolynomialRef.cmp(&ShortlexIntegerPolynomialRef)",
        BenchmarkType::Single,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [("Malachite", &mut |(p, q)| {
            no_out!(ShortlexIntegerPolynomialRef(&p).cmp(&ShortlexIntegerPolynomialRef(&q)));
        })],
    );
}
