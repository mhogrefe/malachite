// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::{ModEvaluateMany, ModEvaluateManyPrecomputed};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_len_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate_many::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_evaluate_many_precomputed
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_evaluate_many_precomputed_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_evaluate_many_precomputed_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_evaluate_many_precomputed_algorithms
    );
}

fn demo_natural_polynomial_mod_evaluate_many_precomputed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, m) in natural_polynomial_natural_vec_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let m_old = m.clone();
        let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
        println!(
            "({p_old}).mod_evaluate_many_precomputed({m_old}, &data({xs:?})) = {:?}",
            p.mod_evaluate_many_precomputed(m, &data)
        );
    }
}

fn demo_natural_polynomial_mod_evaluate_many_precomputed_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, m) in natural_polynomial_natural_vec_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
        println!(
            "(&({p})).mod_evaluate_many_precomputed(&{m}, &data({xs:?})) = {:?}",
            (&p).mod_evaluate_many_precomputed(&m, &data)
        );
    }
}

fn benchmark_natural_polynomial_mod_evaluate_many_precomputed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_evaluate_many_precomputed(Natural, &data)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_vec_natural_triple_gen_var_1()
            .get(gm, config)
            .map(|(p, xs, m)| {
                let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
                (p, m, data)
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_len_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_evaluate_many_precomputed(Natural, &data)",
                &mut |(p, m, data)| {
                    no_out!(p.mod_evaluate_many_precomputed(m, &data));
                },
            ),
            (
                "NaturalPolynomial.mod_evaluate_many_precomputed(&Natural, &data)",
                &mut |(p, m, data)| {
                    no_out!(p.mod_evaluate_many_precomputed(&m, &data));
                },
            ),
            (
                "(&NaturalPolynomial).mod_evaluate_many_precomputed(Natural, &data)",
                &mut |(p, m, data)| {
                    no_out!((&p).mod_evaluate_many_precomputed(m, &data));
                },
            ),
            (
                "(&NaturalPolynomial).mod_evaluate_many_precomputed(&Natural, &data)",
                &mut |(p, m, data)| {
                    no_out!((&p).mod_evaluate_many_precomputed(&m, &data));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_evaluate_many_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_evaluate_many_precomputed(&Natural, &data)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_vec_natural_triple_gen_var_1()
            .get(gm, config)
            .map(|(p, xs, m)| {
                let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
                (p, xs, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_len_bucketer("p"),
        &mut [
            ("precomputed", &mut |(p, _, (m, data))| {
                no_out!((&p).mod_evaluate_many_precomputed(&m, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, xs, (m, _))| {
                    let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
                    no_out!((&p).mod_evaluate_many_precomputed(&m, &data));
                },
            ),
            ("mod_evaluate_many", &mut |(p, xs, (m, _))| {
                no_out!((&p).mod_evaluate_many(&xs, &m));
            }),
            ("naive", &mut |(p, xs, (m, _))| {
                no_out!(mod_evaluate_many_naive(&p, &xs, &m));
            }),
        ],
    );
}
