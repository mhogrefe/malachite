// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_evaluate);
    register_demo!(runner, demo_natural_polynomial_mod_evaluate_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_evaluate_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_evaluate_algorithms);
}

fn demo_natural_polynomial_mod_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p_old, x_old, m_old) = (p.clone(), x.clone(), m.clone());
        println!(
            "({p_old}).mod_evaluate({x_old}, {m_old}) = {}",
            p.mod_evaluate(x, m)
        );
    }
}

fn demo_natural_polynomial_mod_evaluate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate({x}, {m}) = {}",
            (&p).mod_evaluate(&x, &m)
        );
    }
}

fn benchmark_natural_polynomial_mod_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_evaluate(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_evaluate(Natural, Natural)",
                &mut |(p, x, m)| no_out!(p.mod_evaluate(x, m)),
            ),
            (
                "NaturalPolynomial.mod_evaluate(&Natural, &Natural)",
                &mut |(p, x, m)| no_out!(p.mod_evaluate(&x, &m)),
            ),
            (
                "(&NaturalPolynomial).mod_evaluate(Natural, Natural)",
                &mut |(p, x, m)| no_out!((&p).mod_evaluate(x, m)),
            ),
            (
                "(&NaturalPolynomial).mod_evaluate(&Natural, &Natural)",
                &mut |(p, x, m)| no_out!((&p).mod_evaluate(&x, &m)),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_evaluate(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, m)| {
                no_out!((&p).mod_evaluate(&x, &m));
            }),
            ("evaluate, then reduce", &mut |(p, x, m)| {
                no_out!(mod_evaluate_naive(&p, &x, &m));
            }),
        ],
    );
}
