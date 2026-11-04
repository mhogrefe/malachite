// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::{CheckedModGcd, ModGcd, ModGcdAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_5,
};
use malachite_nz::test_util::natural_polynomial::modular::mod_gcd::{
    mod_gcd_euclidean, mod_gcd_hgcd,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_gcd);
    register_demo!(runner, demo_natural_polynomial_mod_gcd_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_gcd_assign);
    register_demo!(runner, demo_natural_polynomial_checked_mod_gcd);
    register_demo!(runner, demo_natural_polynomial_checked_mod_gcd_ref_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_gcd_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_gcd_algorithms);
}

fn show_checked(x: Option<NaturalPolynomial>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!("({p_old}).mod_gcd({q_old}, {m_old}) = {}", p.mod_gcd(q, m));
    }
}

fn demo_natural_polynomial_mod_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_gcd(&({q}), &{m}) = {}", (&p).mod_gcd(&q, &m));
    }
}

fn demo_natural_polynomial_mod_gcd_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        p.mod_gcd_assign(q, m);
        println!("p := {p_old}; p.mod_gcd_assign({q_old}, {m_old}); p = {p}");
    }
}

fn demo_natural_polynomial_checked_mod_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).checked_mod_gcd({q_old}, {m_old}) = {}",
            show_checked(p.checked_mod_gcd(q, m))
        );
    }
}

fn demo_natural_polynomial_checked_mod_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_gcd(&({q}), &{m}) = {}",
            show_checked((&p).checked_mod_gcd(&q, &m))
        );
    }
}

fn benchmark_natural_polynomial_mod_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_gcd(NaturalPolynomial, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.mod_gcd(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_gcd(q, m));
            }),
            ("p.mod_gcd(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_gcd(&q, m));
            }),
            ("(&p).mod_gcd(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_gcd(q, m));
            }),
            ("(&p).mod_gcd(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_gcd(&q, m));
            }),
            ("(&p).mod_gcd(&q, &m)", &mut |(p, q, m)| {
                no_out!((&p).mod_gcd(&q, &m));
            }),
            ("p.mod_gcd_assign(q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_gcd_assign(q, m));
            }),
            ("p.mod_gcd_assign(&q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_gcd_assign(&q, m));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_gcd(NaturalPolynomial, Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_gcd(&q, &m));
            }),
            ("Euclid", &mut |(p, q, m)| {
                no_out!(mod_gcd_euclidean(&p, &q, &m));
            }),
            ("half-GCD", &mut |(p, q, m)| {
                no_out!(mod_gcd_hgcd(&p, &q, &m));
            }),
        ],
    );
}
