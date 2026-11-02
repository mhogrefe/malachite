// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::series::traits::ModCompositionalInverseTruncated;
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_5;
use malachite_nz::test_util::natural_polynomial::series::mod_compositional_inverse_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_compositional_inverse_truncated
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_compositional_inverse_truncated_val_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_compositional_inverse_truncated_ref_val
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_compositional_inverse_truncated_ref_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_compositional_inverse_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_compositional_inverse_truncated_algorithms
    );
}

fn show(p: Option<NaturalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_compositional_inverse_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).mod_compositional_inverse_truncated({len}, {m_old}) = {}",
            show(p.mod_compositional_inverse_truncated(len, m))
        );
    }
}

fn demo_natural_polynomial_mod_compositional_inverse_truncated_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_compositional_inverse_truncated({len}, &{m}) = {}",
            show(p.mod_compositional_inverse_truncated(len, &m))
        );
    }
}

fn demo_natural_polynomial_mod_compositional_inverse_truncated_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        println!(
            "(&({p})).mod_compositional_inverse_truncated({len}, {m_old}) = {}",
            show((&p).mod_compositional_inverse_truncated(len, m))
        );
    }
}

fn demo_natural_polynomial_mod_compositional_inverse_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_compositional_inverse_truncated({len}, &{m}) = {}",
            show((&p).mod_compositional_inverse_truncated(len, &m))
        );
    }
}

fn benchmark_natural_polynomial_mod_compositional_inverse_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_compositional_inverse_truncated(u64, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            (
                "p.mod_compositional_inverse_truncated(len, m)",
                &mut |(p, len, m)| {
                    no_out!(p.mod_compositional_inverse_truncated(len, m));
                },
            ),
            (
                "p.mod_compositional_inverse_truncated(len, &m)",
                &mut |(p, len, m)| {
                    no_out!(p.mod_compositional_inverse_truncated(len, &m));
                },
            ),
            (
                "(&p).mod_compositional_inverse_truncated(len, m)",
                &mut |(p, len, m)| {
                    no_out!((&p).mod_compositional_inverse_truncated(len, m));
                },
            ),
            (
                "(&p).mod_compositional_inverse_truncated(len, &m)",
                &mut |(p, len, m)| {
                    no_out!((&p).mod_compositional_inverse_truncated(len, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_compositional_inverse_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_compositional_inverse_truncated(u64, Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            ("default", &mut |(p, len, m)| {
                no_out!((&p).mod_compositional_inverse_truncated(len, &m));
            }),
            ("naive", &mut |(p, len, m)| {
                no_out!(mod_compositional_inverse_truncated_naive(&p, len, &m));
            }),
            ("Lagrange", &mut |(p, len, m)| {
                no_out!(mod_compositional_inverse_truncated_lagrange(&p, len, &m));
            }),
            ("Lagrange fast", &mut |(p, len, m)| {
                no_out!(mod_compositional_inverse_truncated_lagrange_fast(
                    &p, len, &m
                ));
            }),
            ("Newton", &mut |(p, len, m)| {
                no_out!(mod_compositional_inverse_truncated_newton(&p, len, &m));
            }),
        ],
    );
}
