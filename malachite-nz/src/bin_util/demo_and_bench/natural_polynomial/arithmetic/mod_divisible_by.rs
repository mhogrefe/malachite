// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::ModDivisibleBy;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_divisible_by);
    register_demo!(runner, demo_natural_polynomial_mod_divisible_by_ref_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_divisible_by_evaluation_strategy
    );
}

fn demo_natural_polynomial_mod_divisible_by(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).mod_divisible_by({q_old}, {m_old}) = {}",
            p.mod_divisible_by(q, m)
        );
    }
}

fn demo_natural_polynomial_mod_divisible_by_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_divisible_by(&({q}), &({m})) = {}",
            (&p).mod_divisible_by(&q, &m)
        );
    }
}

fn benchmark_natural_polynomial_mod_divisible_by_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_divisible_by(NaturalPolynomial, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.mod_divisible_by(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_divisible_by(q, m));
            }),
            ("p.mod_divisible_by(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_divisible_by(&q, m));
            }),
            ("(&p).mod_divisible_by(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_divisible_by(q, m));
            }),
            ("(&p).mod_divisible_by(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_divisible_by(&q, m));
            }),
        ],
    );
}
