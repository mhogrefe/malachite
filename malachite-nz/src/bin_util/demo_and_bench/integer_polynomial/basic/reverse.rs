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
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_reverse);
    register_demo!(runner, demo_integer_polynomial_reverse_assign);
    register_bench!(
        runner,
        benchmark_integer_polynomial_reverse_evaluation_strategy
    );
}

fn demo_integer_polynomial_reverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({p}).reverse({len}) = {}", p.reverse(len));
    }
}

fn demo_integer_polynomial_reverse_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, len) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.reverse_assign(len);
        println!("p := {p_old}; p.reverse_assign({len}); p = {p}");
    }
}

fn benchmark_integer_polynomial_reverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.reverse(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("p.reverse(u64)", &mut |(p, len)| no_out!(p.reverse(len))),
            ("p.reverse_assign(u64)", &mut |(mut p, len)| {
                p.reverse_assign(len);
            }),
        ],
    );
}
