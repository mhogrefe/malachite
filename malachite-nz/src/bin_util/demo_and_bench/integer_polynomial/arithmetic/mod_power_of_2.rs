// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, RemPowerOf2, RemPowerOf2Assign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_mod_power_of_2);
    register_demo!(runner, demo_integer_polynomial_mod_power_of_2_ref);

    register_demo!(runner, demo_integer_polynomial_rem_power_of_2);
    register_demo!(runner, demo_integer_polynomial_rem_power_of_2_ref);
    register_demo!(runner, demo_integer_polynomial_rem_power_of_2_assign);

    register_bench!(
        runner,
        benchmark_integer_polynomial_mod_power_of_2_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_rem_power_of_2_evaluation_strategy
    );
}

fn demo_integer_polynomial_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2({pow}) = {}",
            p.mod_power_of_2(pow)
        );
    }
}

fn demo_integer_polynomial_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2({pow}) = {}",
            (&p).mod_power_of_2(pow)
        );
    }
}

fn benchmark_integer_polynomial_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.mod_power_of_2(u64)", &mut |(p, pow)| {
                no_out!(p.mod_power_of_2(pow));
            }),
            (
                "(&IntegerPolynomial).mod_power_of_2(u64)",
                &mut |(p, pow)| {
                    no_out!((&p).mod_power_of_2(pow));
                },
            ),
        ],
    );
}

fn demo_integer_polynomial_rem_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).rem_power_of_2({pow}) = {}",
            p.rem_power_of_2(pow)
        );
    }
}

fn demo_integer_polynomial_rem_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).rem_power_of_2({pow}) = {}",
            (&p).rem_power_of_2(pow)
        );
    }
}

fn demo_integer_polynomial_rem_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, pow) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.rem_power_of_2_assign(pow);
        println!("p := {p_old}; p.rem_power_of_2_assign({pow}); p = {p}");
    }
}

fn benchmark_integer_polynomial_rem_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.rem_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.rem_power_of_2(u64)", &mut |(p, pow)| {
                no_out!(p.rem_power_of_2(pow));
            }),
            (
                "(&IntegerPolynomial).rem_power_of_2(u64)",
                &mut |(p, pow)| {
                    no_out!((&p).rem_power_of_2(pow));
                },
            ),
            (
                "IntegerPolynomial.rem_power_of_2_assign(u64)",
                &mut |(mut p, pow)| p.rem_power_of_2_assign(pow),
            ),
        ],
    );
}
