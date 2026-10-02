// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{SquareTruncated, SquareTruncatedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_1;
use malachite_q::test_util::rational_polynomial::arithmetic::square_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_square_truncated);
    register_demo!(runner, demo_rational_polynomial_square_truncated_ref);
    register_demo!(runner, demo_rational_polynomial_square_truncated_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_square_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_square_truncated_algorithms
    );
}

fn demo_rational_polynomial_square_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).square_truncated({len}) = {}",
            p.square_truncated(len)
        );
    }
}

fn demo_rational_polynomial_square_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).square_truncated({len}) = {}",
            (&p).square_truncated(len)
        );
    }
}

fn demo_rational_polynomial_square_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, len) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.square_truncated_assign(len);
        println!("p := {p_old}; p.square_truncated_assign({len}); p = {p}");
    }
}

fn benchmark_rational_polynomial_square_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.square_truncated(len)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("p.square_truncated(len)", &mut |(p, len)| {
                no_out!(p.square_truncated(len));
            }),
            ("(&p).square_truncated(len)", &mut |(p, len)| {
                no_out!((&p).square_truncated(len));
            }),
            ("p.square_truncated_assign(len)", &mut |(mut p, len)| {
                p.square_truncated_assign(len);
            }),
        ],
    );
}

fn benchmark_rational_polynomial_square_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "p.square_truncated(len)",
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len)| no_out!(p.square_truncated(len))),
            ("naive", &mut |(p, len)| {
                no_out!(square_truncated_naive(&p, len));
            }),
        ],
    );
}
