// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Neg, ModPowerOf2NegAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_unsigned_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_neg);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_neg_ref);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_neg_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_neg_evaluation_strategy
    );
}

fn demo_natural_polynomial_mod_power_of_2_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_neg({pow}) = {}",
            p.mod_power_of_2_neg(pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_neg({pow}) = {}",
            (&p).mod_power_of_2_neg(pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_neg_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, _, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_neg_assign(pow);
        println!("p := {p_old}; p.mod_power_of_2_neg_assign({pow}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_neg(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_power_of_2_neg(u64)",
                &mut |(p, _, pow)| {
                    no_out!(p.mod_power_of_2_neg(pow));
                },
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_neg(u64)",
                &mut |(p, _, pow)| {
                    no_out!((&p).mod_power_of_2_neg(pow));
                },
            ),
            (
                "NaturalPolynomial.mod_power_of_2_neg_assign(u64)",
                &mut |(mut p, _, pow)| {
                    p.mod_power_of_2_neg_assign(pow);
                },
            ),
        ],
    );
}
