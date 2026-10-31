// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRem, ModDivRemPrecomputed};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_div_rem_precomputed);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_div_rem_precomputed_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_div_assign_rem_precomputed
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_div_rem_precomputed_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_div_rem_precomputed_algorithms
    );
}

fn show((q, r): (NaturalPolynomial, NaturalPolynomial)) -> String {
    format!("({q}, {r})")
}

fn demo_natural_polynomial_mod_div_rem_precomputed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
        println!(
            "({p_old}).mod_div_rem_precomputed({q_old}, {m_old}, &data) = {}",
            show(p.mod_div_rem_precomputed(q, m, &data))
        );
    }
}

fn demo_natural_polynomial_mod_div_rem_precomputed_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
        println!(
            "(&({p})).mod_div_rem_precomputed(&({q}), {m}, &data) = {}",
            show((&p).mod_div_rem_precomputed(&q, &m, &data))
        );
    }
}

fn demo_natural_polynomial_mod_div_assign_rem_precomputed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
        let r = p.mod_div_assign_rem_precomputed(q, m, &data);
        println!(
            "p := {p_old}; p.mod_div_assign_rem_precomputed({q_old}, {m_old}, &data) = {r}; p = {p}"
        );
    }
}

fn benchmark_natural_polynomial_mod_div_rem_precomputed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_div_rem_precomputed(NaturalPolynomial, Natural, &data)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
            .get(gm, config)
            .map(|(p, q, m)| {
                let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
                (p, q, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "p.mod_div_rem_precomputed(q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!(p.mod_div_rem_precomputed(q, m, &data));
                },
            ),
            (
                "p.mod_div_rem_precomputed(&q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!(p.mod_div_rem_precomputed(&q, m, &data));
                },
            ),
            (
                "(&p).mod_div_rem_precomputed(q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!((&p).mod_div_rem_precomputed(q, m, &data));
                },
            ),
            (
                "(&p).mod_div_rem_precomputed(&q, &m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!((&p).mod_div_rem_precomputed(&q, &m, &data));
                },
            ),
            (
                "p.mod_div_assign_rem_precomputed(q, m, &data)",
                &mut |(mut p, q, (m, data))| {
                    no_out!(p.mod_div_assign_rem_precomputed(q, m, &data));
                },
            ),
            (
                "p.mod_div_assign_rem_precomputed(&q, m, &data)",
                &mut |(mut p, q, (m, data))| {
                    no_out!(p.mod_div_assign_rem_precomputed(&q, m, &data));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_div_rem_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_div_rem_precomputed(NaturalPolynomial, Natural, &data)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
            .get(gm, config)
            .map(|(p, q, m)| {
                let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
                (p, q, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("precomputed", &mut |(p, q, (m, data))| {
                no_out!((&p).mod_div_rem_precomputed(&q, &m, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, q, (m, _))| {
                    let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
                    no_out!((&p).mod_div_rem_precomputed(&q, &m, &data));
                },
            ),
            ("not precomputed", &mut |(p, q, (m, _))| {
                no_out!((&p).mod_div_rem(&q, &m));
            }),
        ],
    );
}
