// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Sub, ModPowerOf2SubAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_sub::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_sub);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_sub_val_ref);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_sub_ref_val);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_sub_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_sub_assign);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_sub_assign_ref
    );

    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_sub_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_sub_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_sub_assign_evaluation_strategy
    );
}

fn demo_natural_polynomial_mod_power_of_2_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_sub({q_old}, {pow}) = {}",
            p.mod_power_of_2_sub(q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_sub_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_sub(&({q}), {pow}) = {}",
            p.mod_power_of_2_sub(&q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_sub_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mod_power_of_2_sub({q_old}, {pow}) = {}",
            (&p).mod_power_of_2_sub(q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_sub_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_sub(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_sub(&q, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_sub_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_sub_assign(q, pow);
        println!("p := {p_old}; p.mod_power_of_2_sub_assign({q_old}, {pow}); p = {p}");
    }
}

fn demo_natural_polynomial_mod_power_of_2_sub_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_sub_assign(&q, pow);
        println!("p := {p_old}; p.mod_power_of_2_sub_assign(&({q}), {pow}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_sub(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mod_power_of_2_sub(NaturalPolynomial, u64)",
                &mut |(p, q, pow)| {
                    no_out!(p.mod_power_of_2_sub(q, pow));
                },
            ),
            (
                "NaturalPolynomial.mod_power_of_2_sub(&NaturalPolynomial, u64)",
                &mut |(p, q, pow)| {
                    no_out!(p.mod_power_of_2_sub(&q, pow));
                },
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_sub(NaturalPolynomial, u64)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_sub(q, pow));
                },
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_sub(&NaturalPolynomial, u64)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_sub(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_sub_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_sub(NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_sub(q, pow));
            }),
            ("naive", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_sub_naive(&p, &q, pow));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_sub_assign(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mod_power_of_2_sub_assign(NaturalPolynomial, u64)",
                &mut |(mut p, q, pow)| p.mod_power_of_2_sub_assign(q, pow),
            ),
            (
                "NaturalPolynomial.mod_power_of_2_sub_assign(&NaturalPolynomial, u64)",
                &mut |(mut p, q, pow)| p.mod_power_of_2_sub_assign(&q, pow),
            ),
        ],
    );
}
