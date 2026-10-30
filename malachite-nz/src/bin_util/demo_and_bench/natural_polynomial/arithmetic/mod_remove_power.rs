// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModRemovePower, ModRemovePowerAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_remove_power::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_remove_power);
    register_demo!(runner, demo_natural_polynomial_mod_remove_power_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_remove_power_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_remove_power_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_remove_power_algorithms
    );
}

fn demo_natural_polynomial_mod_remove_power(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        let (s, e) = p.mod_remove_power(q, m);
        println!("({p_old}).mod_remove_power({q_old}, {m_old}) = ({s}, {e})");
    }
}

fn demo_natural_polynomial_mod_remove_power_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (s, e) = (&p).mod_remove_power(&q, &m);
        println!("(&({p})).mod_remove_power(&({q}), &({m})) = ({s}, {e})");
    }
}

fn demo_natural_polynomial_mod_remove_power_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        let e = p.mod_remove_power_assign(q, m);
        println!("p := {p_old}; p.mod_remove_power_assign({q_old}, {m_old}) = {e}; p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_remove_power_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_remove_power(NaturalPolynomial, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.mod_remove_power(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_remove_power(q, m));
            }),
            ("p.mod_remove_power(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_remove_power(&q, m));
            }),
            ("(&p).mod_remove_power(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_remove_power(q, m));
            }),
            ("(&p).mod_remove_power(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_remove_power(&q, m));
            }),
            ("p.mod_remove_power_assign(q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_remove_power_assign(q, m));
            }),
            ("p.mod_remove_power_assign(&q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_remove_power_assign(&q, m));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_remove_power_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_remove_power(NaturalPolynomial, Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_remove_power(&q, &m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_remove_power_naive(&p, &q, &m));
            }),
        ],
    );
}
