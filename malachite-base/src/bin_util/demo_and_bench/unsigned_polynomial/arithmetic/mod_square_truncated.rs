// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModSquareTruncated, ModSquareTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_square_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_square_truncated);
    register_demo!(runner, demo_unsigned_polynomial_mod_square_truncated_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_square_truncated_assign);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_square_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_square_truncated_algorithms
    );
}

fn demo_unsigned_polynomial_mod_square_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_square_truncated({len}, {m}) = {}",
            p.mod_square_truncated(len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_square_truncated_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, _, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_square_truncated({len}, {m}) = {}",
            (&p).mod_square_truncated(len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_square_truncated_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, _, len, m) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        p.mod_square_truncated_assign(len, m);
        println!("p := {p_old}; p.mod_square_truncated_assign({len}, {m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_square_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_square_truncated(len, m)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_square_truncated(len, m)", &mut |(p, _, len, m)| {
                no_out!(p.mod_square_truncated(len, m));
            }),
            ("(&p).mod_square_truncated(len, m)", &mut |(
                p,
                _,
                len,
                m,
            )| {
                no_out!((&p).mod_square_truncated(len, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_square_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.mod_square_truncated(len, m)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, _, len, m)| {
                no_out!(p.mod_square_truncated(len, m));
            }),
            ("naive", &mut |(p, _, len, m)| {
                no_out!(mod_square_truncated_polynomial_naive(&p, len, m));
            }),
        ],
    );
}
