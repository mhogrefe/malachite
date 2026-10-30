// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{
    CheckedModPowerOf2DivExact, ModPowerOf2DivExact, ModPowerOf2DivExactAssign, ModPowerOf2Quotient,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5,
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_exact::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_div_exact);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_div_exact_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_div_exact_assign
    );
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_power_of_2_div_exact
    );
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_power_of_2_div_exact_ref_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_div_exact_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_div_exact_algorithms
    );
}

fn show_checked(x: Option<NaturalPolynomial>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_power_of_2_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_div_exact({q_old}, {pow}) = {}",
            p.mod_power_of_2_div_exact(q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_div_exact_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_div_exact(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_div_exact(&q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_div_exact_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_div_exact_assign(q, pow);
        println!("p := {p_old}; p.mod_power_of_2_div_exact_assign({q_old}, {pow}); p = {p}");
    }
}

fn demo_natural_polynomial_checked_mod_power_of_2_div_exact(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_power_of_2_div_exact({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_div_exact(q, pow))
        );
    }
}

fn demo_natural_polynomial_checked_mod_power_of_2_div_exact_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_div_exact(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_div_exact(&q, pow))
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_div_exact(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.mod_power_of_2_div_exact(q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_div_exact(q, pow));
            }),
            ("p.mod_power_of_2_div_exact(&q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_div_exact(&q, pow));
            }),
            (
                "(&p).mod_power_of_2_div_exact(q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_div_exact(q, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_div_exact(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_div_exact(&q, pow));
                },
            ),
            (
                "p.mod_power_of_2_div_exact_assign(q, pow)",
                &mut |(mut p, q, pow)| {
                    no_out!(p.mod_power_of_2_div_exact_assign(q, pow));
                },
            ),
            (
                "p.mod_power_of_2_div_exact_assign(&q, pow)",
                &mut |(mut p, q, pow)| {
                    no_out!(p.mod_power_of_2_div_exact_assign(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_div_exact(NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_div_exact(&q, pow));
            }),
            ("from the top", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_div_exact_top_wrapper(&p, &q, pow));
            }),
            ("bidirectional basecase", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_div_exact_bidirectional_basecase_wrapper(
                    &p, &q, pow
                ));
            }),
            ("bidirectional Karp-Markstein", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_div_exact_bidirectional_newton_wrapper(
                    &p, &q, pow
                ));
            }),
            ("using quotient", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_quotient(&q, pow));
            }),
        ],
    );
}
