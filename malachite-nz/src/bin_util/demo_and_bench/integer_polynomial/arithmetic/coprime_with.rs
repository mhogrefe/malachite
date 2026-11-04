// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::number_theory::traits::CoprimeWith;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_7;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_coprime_with);
    register_demo!(runner, demo_integer_polynomial_coprime_with_ref_ref);
    register_bench!(
        runner,
        benchmark_integer_polynomial_coprime_with_evaluation_strategy
    );
}

fn demo_integer_polynomial_coprime_with(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).coprime_with({q_old}) = {}", p.coprime_with(q));
    }
}

fn demo_integer_polynomial_coprime_with_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).coprime_with(&({q})) = {}", (&p).coprime_with(&q));
    }
}

fn benchmark_integer_polynomial_coprime_with_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.coprime_with(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.coprime_with(q)", &mut |(p, q)| {
                no_out!(p.coprime_with(q));
            }),
            ("p.coprime_with(&q)", &mut |(p, q)| {
                no_out!(p.coprime_with(&q));
            }),
            ("(&p).coprime_with(q)", &mut |(p, q)| {
                no_out!((&p).coprime_with(q));
            }),
            ("(&p).coprime_with(&q)", &mut |(p, q)| {
                no_out!((&p).coprime_with(&q));
            }),
        ],
    );
}
