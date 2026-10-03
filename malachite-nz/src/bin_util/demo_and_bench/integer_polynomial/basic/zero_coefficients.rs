// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_unsigned_triple_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_zero_coefficients);
    register_bench!(runner, benchmark_integer_polynomial_zero_coefficients);
}

fn demo_integer_polynomial_zero_coefficients(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, start, end) in integer_polynomial_unsigned_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.zero_coefficients(start, end);
        println!("p := {p_old}; p.zero_coefficients({start}, {end}); p = {p}");
    }
}

fn benchmark_integer_polynomial_zero_coefficients(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.zero_coefficients(u64, u64)",
        BenchmarkType::Single,
        integer_polynomial_unsigned_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(mut p, start, end)| {
            p.zero_coefficients(start, end);
        })],
    );
}
