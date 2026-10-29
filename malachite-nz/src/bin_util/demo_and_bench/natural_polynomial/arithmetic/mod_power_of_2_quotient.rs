// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{
    CheckedModPowerOf2Quotient, ModPowerOf2DivRem, ModPowerOf2Quotient, ModPowerOf2QuotientAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1,
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_quotient);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_quotient_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_quotient_assign
    );
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_power_of_2_quotient
    );
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_power_of_2_quotient_ref_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_quotient_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_quotient_algorithms
    );
}

fn show_checked(x: Option<NaturalPolynomial>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_power_of_2_quotient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_quotient({q_old}, {pow}) = {}",
            p.mod_power_of_2_quotient(q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_quotient_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_quotient(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_quotient(&q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_quotient_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_quotient_assign(q, pow);
        println!("p := {p_old}; p.mod_power_of_2_quotient_assign({q_old}, {pow}); p = {p}");
    }
}

fn demo_natural_polynomial_checked_mod_power_of_2_quotient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_power_of_2_quotient({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_quotient(q, pow))
        );
    }
}

fn demo_natural_polynomial_checked_mod_power_of_2_quotient_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_quotient(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_quotient(&q, pow))
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_quotient_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_quotient(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.mod_power_of_2_quotient(q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_quotient(q, pow));
            }),
            ("p.mod_power_of_2_quotient(&q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_quotient(&q, pow));
            }),
            ("(&p).mod_power_of_2_quotient(q, pow)", &mut |(
                p,
                q,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_quotient(q, pow));
            }),
            (
                "(&p).mod_power_of_2_quotient(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_quotient(&q, pow));
                },
            ),
            (
                "p.mod_power_of_2_quotient_assign(q, pow)",
                &mut |(mut p, q, pow)| {
                    no_out!(p.mod_power_of_2_quotient_assign(q, pow));
                },
            ),
            (
                "p.mod_power_of_2_quotient_assign(&q, pow)",
                &mut |(mut p, q, pow)| {
                    no_out!(p.mod_power_of_2_quotient_assign(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_quotient_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_quotient(NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_quotient(&q, pow));
            }),
            ("using div_rem", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_div_rem(&q, pow).0);
            }),
        ],
    );
}
