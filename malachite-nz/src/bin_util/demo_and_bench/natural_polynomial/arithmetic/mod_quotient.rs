// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{CheckedModQuotient, ModDivRem, ModQuotient, ModQuotientAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_quotient);
    register_demo!(runner, demo_natural_polynomial_mod_quotient_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_quotient_assign);
    register_demo!(runner, demo_natural_polynomial_checked_mod_quotient);
    register_demo!(runner, demo_natural_polynomial_checked_mod_quotient_ref_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_quotient_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_quotient_algorithms);
}

fn show_checked(x: Option<NaturalPolynomial>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_quotient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).mod_quotient({q_old}, {m_old}) = {}",
            p.mod_quotient(q, m)
        );
    }
}

fn demo_natural_polynomial_mod_quotient_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_quotient(&({q}), &({m})) = {}",
            (&p).mod_quotient(&q, &m)
        );
    }
}

fn demo_natural_polynomial_mod_quotient_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        p.mod_quotient_assign(q, m);
        println!("p := {p_old}; p.mod_quotient_assign({q_old}, {m_old}); p = {p}");
    }
}

fn demo_natural_polynomial_checked_mod_quotient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).checked_mod_quotient({q_old}, {m_old}) = {}",
            show_checked(p.checked_mod_quotient(q, m))
        );
    }
}

fn demo_natural_polynomial_checked_mod_quotient_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_quotient(&({q}), &({m})) = {}",
            show_checked((&p).checked_mod_quotient(&q, &m))
        );
    }
}

fn benchmark_natural_polynomial_mod_quotient_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_quotient(NaturalPolynomial, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.mod_quotient(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_quotient(q, m));
            }),
            ("p.mod_quotient(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_quotient(&q, m));
            }),
            ("(&p).mod_quotient(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(q, m));
            }),
            ("(&p).mod_quotient(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(&q, m));
            }),
            ("(&p).mod_quotient(&q, &m)", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(&q, &m));
            }),
            ("p.mod_quotient_assign(q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_quotient_assign(q, m));
            }),
            ("p.mod_quotient_assign(&q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_quotient_assign(&q, m));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_quotient_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_quotient(NaturalPolynomial, Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(&q, &m));
            }),
            ("using div_rem", &mut |(p, q, m)| {
                no_out!((&p).mod_div_rem(&q, &m).0);
            }),
        ],
    );
}
