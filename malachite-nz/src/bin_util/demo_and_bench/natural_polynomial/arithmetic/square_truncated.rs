// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{MulTruncated, SquareTruncated, SquareTruncatedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_pair_gen_var_3;
use malachite_nz::test_util::natural_polynomial::arithmetic::square_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_square_truncated);
    register_demo!(runner, demo_natural_polynomial_square_truncated_ref);
    register_demo!(runner, demo_natural_polynomial_square_truncated_assign);

    register_bench!(
        runner,
        benchmark_natural_polynomial_square_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_square_truncated_algorithms
    );
}

fn demo_natural_polynomial_square_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in natural_polynomial_unsigned_pair_gen_var_3::<u64>()
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

fn demo_natural_polynomial_square_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in natural_polynomial_unsigned_pair_gen_var_3::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).square_truncated({len}) = {}",
            (&p).square_truncated(len)
        );
    }
}

fn demo_natural_polynomial_square_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, len) in natural_polynomial_unsigned_pair_gen_var_3::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.square_truncated_assign(len);
        println!("p := {p_old}; p.square_truncated_assign({len}); p = {p}");
    }
}

fn benchmark_natural_polynomial_square_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.square_truncated(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_pair_gen_var_3::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.square_truncated(u64)",
                &mut |(p, len)| {
                    no_out!(p.square_truncated(len));
                },
            ),
            (
                "(&NaturalPolynomial).square_truncated(u64)",
                &mut |(p, len)| {
                    no_out!((&p).square_truncated(len));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_square_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.square_truncated(u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_pair_gen_var_3::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len)| {
                no_out!(p.square_truncated(len));
            }),
            ("using mul_truncated", &mut |(p, len)| {
                no_out!((&p).mul_truncated(&p, len));
            }),
            ("naive", &mut |(p, len)| {
                no_out!(square_truncated_naive(&p, len));
            }),
        ],
    );
}
