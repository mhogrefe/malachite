// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_divisible_by);
    register_demo!(runner, demo_integer_polynomial_divisible_by_ref_ref);
    register_bench!(
        runner,
        benchmark_integer_polynomial_divisible_by_evaluation_strategy
    );
}

fn demo_integer_polynomial_divisible_by(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).divisible_by({q_old}) = {}", p.divisible_by(q));
    }
}

fn demo_integer_polynomial_divisible_by_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).divisible_by(&({q})) = {}", (&p).divisible_by(&q));
    }
}

fn benchmark_integer_polynomial_divisible_by_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.divisible_by(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.divisible_by(q)", &mut |(p, q)| {
                no_out!(p.divisible_by(q));
            }),
            ("p.divisible_by(&q)", &mut |(p, q)| {
                no_out!(p.divisible_by(&q));
            }),
            ("(&p).divisible_by(q)", &mut |(p, q)| {
                no_out!((&p).divisible_by(q));
            }),
            ("(&p).divisible_by(&q)", &mut |(p, q)| {
                no_out!((&p).divisible_by(&q));
            }),
        ],
    );
}
