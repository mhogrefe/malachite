// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_mul);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_mul_val_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_mul_ref_val);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_mul_ref_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_mul_assign);
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_mul_assign_ref
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_mul_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_mul_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_mul_assign_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_mul({q_old}, {pow}) = {}",
            p.mod_power_of_2_mul(q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_mul_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_mul(&({q}), {pow}) = {}",
            p.mod_power_of_2_mul(&q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_mul_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mod_power_of_2_mul({q_old}, {pow}) = {}",
            (&p).mod_power_of_2_mul(q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_mul_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_mul(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_mul(&q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_mul_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_mul_assign(q, pow);
        println!("p := {p_old}; p.mod_power_of_2_mul_assign({q_old}, {pow}); p = {p}");
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_mul_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_mul_assign(&q, pow);
        println!("p := {p_old}; p.mod_power_of_2_mul_assign(&({q}), {pow}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_mul(UnsignedPolynomial<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_power_of_2_mul(UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, pow)| {
                    no_out!(p.mod_power_of_2_mul(q, pow));
                },
            ),
            (
                "UnsignedPolynomial<u64>.mod_power_of_2_mul(&UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, pow)| {
                    no_out!(p.mod_power_of_2_mul(&q, pow));
                },
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_power_of_2_mul(UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_mul(q, pow));
                },
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_power_of_2_mul(&UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_mul(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_mul(UnsignedPolynomial<u64>, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_mul(q, pow));
            }),
            ("naive", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_mul_polynomial_naive(&p, &q, pow));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_mul_assign(UnsignedPolynomial<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_power_of_2_mul_assign(UnsignedPolynomial<u64>, u64)",
                &mut |(mut p, q, pow)| {
                    p.mod_power_of_2_mul_assign(q, pow);
                },
            ),
            (
                "UnsignedPolynomial<u64>.mod_power_of_2_mul_assign(&UnsignedPolynomial<u64>, u64)",
                &mut |(mut p, q, pow)| {
                    p.mod_power_of_2_mul_assign(&q, pow);
                },
            ),
        ],
    );
}
