// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::gaussian_integer::factorization::remove_one_plus_i::*;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_remove_one_plus_i);
    register_demo!(runner, demo_gaussian_integer_remove_one_plus_i_assign);

    register_bench!(
        runner,
        benchmark_gaussian_integer_remove_one_plus_i_algorithms
    );
    register_bench!(
        runner,
        benchmark_gaussian_integer_remove_one_plus_i_evaluation_strategy
    );
}

fn demo_gaussian_integer_remove_one_plus_i(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let (q, k) = x.remove_one_plus_i();
        println!("({x}).remove_one_plus_i() = ({q}, {k})");
    }
}

fn demo_gaussian_integer_remove_one_plus_i_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in gaussian_integer_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let k = x.remove_one_plus_i_assign();
        println!("x := {x_old}; x.remove_one_plus_i_assign() = {k}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_remove_one_plus_i_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.remove_one_plus_i()",
        BenchmarkType::Algorithms,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                no_out!(x.remove_one_plus_i());
            }),
            ("naive", &mut |x| {
                no_out!(gaussian_integer_remove_one_plus_i_naive(&x));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_remove_one_plus_i_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.remove_one_plus_i()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.remove_one_plus_i()", &mut |x| {
                no_out!(x.remove_one_plus_i());
            }),
            (
                "GaussianInteger.remove_one_plus_i_assign()",
                &mut |mut x| {
                    no_out!(x.remove_one_plus_i_assign());
                },
            ),
        ],
    );
}
