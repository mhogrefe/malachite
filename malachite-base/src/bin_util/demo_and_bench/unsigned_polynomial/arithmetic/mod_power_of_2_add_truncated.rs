// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModPowerOf2AddTruncated, ModPowerOf2AddTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_add_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_add_truncated
    );
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_add_truncated_val_ref
    );
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_add_truncated_ref_val
    );
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_add_truncated_ref_ref
    );
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_add_truncated_assign
    );
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_add_truncated_assign_ref
    );

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_add_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_add_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_add_truncated_assign_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_add_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_add_truncated({q_old}, {len}, {pow}) = {}",
            p.mod_power_of_2_add_truncated(q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_add_truncated_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_add_truncated(&({q}), {len}, {pow}) = {}",
            p.mod_power_of_2_add_truncated(&q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_add_truncated_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mod_power_of_2_add_truncated({q_old}, {len}, {pow}) = {}",
            (&p).mod_power_of_2_add_truncated(q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_add_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_add_truncated(&({q}), {len}, {pow}) = {}",
            (&p).mod_power_of_2_add_truncated(&q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_add_truncated_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, pow) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_add_truncated_assign(q, len, pow);
        println!(
            "p := {p_old}; p.mod_power_of_2_add_truncated_assign({q_old}, {len}, {pow}); p = {p}"
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_add_truncated_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, pow) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_add_truncated_assign(&q, len, pow);
        println!(
            "p := {p_old}; p.mod_power_of_2_add_truncated_assign(&({q}), {len}, {pow}); p = {p}"
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_add_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_power_of_2_add_truncated(q, len, pow)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "p.mod_power_of_2_add_truncated(q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!(p.mod_power_of_2_add_truncated(q, len, pow));
                },
            ),
            (
                "p.mod_power_of_2_add_truncated(&q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!(p.mod_power_of_2_add_truncated(&q, len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_add_truncated(q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!((&p).mod_power_of_2_add_truncated(q, len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_add_truncated(&q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!((&p).mod_power_of_2_add_truncated(&q, len, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_add_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_power_of_2_add_truncated(q, len, pow)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len, pow)| {
                no_out!(p.mod_power_of_2_add_truncated(q, len, pow));
            }),
            ("naive", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_add_truncated_naive(&p, &q, len, pow));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_add_truncated_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_power_of_2_add_truncated_assign(q, len, pow)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "mod_power_of_2_add_truncated_assign(q, len, pow)",
                &mut |(mut p, q, len, pow)| p.mod_power_of_2_add_truncated_assign(q, len, pow),
            ),
            (
                "mod_power_of_2_add_truncated_assign(&q, len, pow)",
                &mut |(mut p, q, len, pow)| p.mod_power_of_2_add_truncated_assign(&q, len, pow),
            ),
        ],
    );
}
