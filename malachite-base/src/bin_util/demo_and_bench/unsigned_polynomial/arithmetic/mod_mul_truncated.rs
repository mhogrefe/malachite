// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModMulTruncated, ModMulTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_mul_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_mul_truncated);
    register_demo!(runner, demo_unsigned_polynomial_mod_mul_truncated_val_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_mul_truncated_ref_val);
    register_demo!(runner, demo_unsigned_polynomial_mod_mul_truncated_ref_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_mul_truncated_assign);
    register_demo!(
        runner,
        demo_unsigned_polynomial_mod_mul_truncated_assign_ref
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_mul_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_mul_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_mul_truncated_assign_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_mul_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_mul_truncated({q_old}, {len}, {m}) = {}",
            p.mod_mul_truncated(q, len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_mul_truncated_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_mul_truncated(&({q}), {len}, {m}) = {}",
            p.mod_mul_truncated(&q, len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_mul_truncated_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mod_mul_truncated({q_old}, {len}, {m}) = {}",
            (&p).mod_mul_truncated(q, len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_mul_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_mul_truncated(&({q}), {len}, {m}) = {}",
            (&p).mod_mul_truncated(&q, len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_mul_truncated_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, m) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_mul_truncated_assign(q, len, m);
        println!("p := {p_old}; p.mod_mul_truncated_assign({q_old}, {len}, {m}); p = {p}");
    }
}

fn demo_unsigned_polynomial_mod_mul_truncated_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, m) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        p.mod_mul_truncated_assign(&q, len, m);
        println!("p := {p_old}; p.mod_mul_truncated_assign(&({q}), {len}, {m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_mul_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_mul_truncated(q, len, m)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_mul_truncated(q, len, m)", &mut |(p, q, len, m)| {
                no_out!(p.mod_mul_truncated(q, len, m));
            }),
            ("p.mod_mul_truncated(&q, len, m)", &mut |(p, q, len, m)| {
                no_out!(p.mod_mul_truncated(&q, len, m));
            }),
            ("(&p).mod_mul_truncated(q, len, m)", &mut |(
                p,
                q,
                len,
                m,
            )| {
                no_out!((&p).mod_mul_truncated(q, len, m));
            }),
            ("(&p).mod_mul_truncated(&q, len, m)", &mut |(
                p,
                q,
                len,
                m,
            )| {
                no_out!((&p).mod_mul_truncated(&q, len, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_mul_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_mul_truncated(q, len, m)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len, m)| {
                no_out!(p.mod_mul_truncated(q, len, m));
            }),
            ("naive", &mut |(p, q, len, m)| {
                no_out!(mod_mul_truncated_polynomial_naive(&p, &q, len, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_mul_truncated_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_mul_truncated_assign(q, len, m)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "p.mod_mul_truncated_assign(q, len, m)",
                &mut |(mut p, q, len, m)| {
                    p.mod_mul_truncated_assign(q, len, m);
                },
            ),
            (
                "p.mod_mul_truncated_assign(&q, len, m)",
                &mut |(mut p, q, len, m)| {
                    p.mod_mul_truncated_assign(&q, len, m);
                },
            ),
        ],
    );
}
