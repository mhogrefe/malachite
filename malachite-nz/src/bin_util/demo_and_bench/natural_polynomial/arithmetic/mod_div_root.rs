// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModDivRoot, ModDivRootAssign, ModEvaluate};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_div_root::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_div_root);
    register_demo!(runner, demo_natural_polynomial_mod_div_root_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_div_root_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_div_root_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_div_root_algorithms);
}

fn demo_natural_polynomial_mod_div_root(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let c_old = c.clone();
        let m_old = m.clone();
        let (q, r) = p.mod_div_root(c, m);
        println!("({p_old}).mod_div_root({c_old}, {m_old}) = ({q}, {r})");
    }
}

fn demo_natural_polynomial_mod_div_root_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, c, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (q, r) = (&p).mod_div_root(&c, &m);
        println!("(&({p})).mod_div_root(&({c}), &({m})) = ({q}, {r})");
    }
}

fn demo_natural_polynomial_mod_div_root_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, c, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let c_old = c.clone();
        let m_old = m.clone();
        let r = p.mod_div_root_assign(c, m);
        println!("p := {p_old}; p.mod_div_root_assign({c_old}, {m_old}) = {r}; p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_div_root_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_div_root(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("p.mod_div_root(c, m)", &mut |(p, c, m)| {
                no_out!(p.mod_div_root(c, m));
            }),
            ("p.mod_div_root(&c, m)", &mut |(p, c, m)| {
                no_out!(p.mod_div_root(&c, m));
            }),
            ("(&p).mod_div_root(c, m)", &mut |(p, c, m)| {
                no_out!((&p).mod_div_root(c, m));
            }),
            ("(&p).mod_div_root(&c, m)", &mut |(p, c, m)| {
                no_out!((&p).mod_div_root(&c, m));
            }),
            ("p.mod_div_root_assign(c, m)", &mut |(mut p, c, m)| {
                no_out!(p.mod_div_root_assign(c, m));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_div_root_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_div_root(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, m)| {
                no_out!((&p).mod_div_root(&c, &m));
            }),
            ("naive", &mut |(p, c, m)| {
                no_out!(mod_div_root_naive(&p, &c, &m));
            }),
            ("evaluation only", &mut |(p, c, m)| {
                no_out!((&p).mod_evaluate(&c, &m));
            }),
        ],
    );
}
