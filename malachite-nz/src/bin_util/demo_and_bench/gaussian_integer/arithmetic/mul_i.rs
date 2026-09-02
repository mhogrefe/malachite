// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulI, MulIAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_mul_i);
    register_demo!(runner, demo_gaussian_integer_mul_i_ref);
    register_demo!(runner, demo_gaussian_integer_mul_i_assign);

    register_bench!(runner, benchmark_gaussian_integer_mul_i_evaluation_strategy);
}

fn demo_gaussian_integer_mul_i(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("({}).mul_i() = {}", x.clone(), x.mul_i());
    }
}

fn demo_gaussian_integer_mul_i_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("(&{}).mul_i() = {}", x, (&x).mul_i());
    }
}

fn demo_gaussian_integer_mul_i_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in gaussian_integer_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.mul_i_assign();
        println!("x := {old_x}; x.mul_i_assign(); x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_mul_i_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.mul_i()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.mul_i()", &mut |x| {
                no_out!(x.mul_i());
            }),
            ("(&GaussianInteger).mul_i()", &mut |x| {
                no_out!((&x).mul_i());
            }),
        ],
    );
}
