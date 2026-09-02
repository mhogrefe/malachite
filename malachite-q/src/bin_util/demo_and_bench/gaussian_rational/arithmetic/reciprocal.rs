// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, Conjugate, Reciprocal, ReciprocalAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_reciprocal);
    register_demo!(runner, demo_gaussian_rational_reciprocal_ref);
    register_demo!(runner, demo_gaussian_rational_reciprocal_assign);

    register_bench!(runner, benchmark_gaussian_rational_reciprocal_algorithms);
    register_bench!(
        runner,
        benchmark_gaussian_rational_reciprocal_evaluation_strategy
    );
}

fn demo_gaussian_rational_reciprocal(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen_var_3().get(gm, config).take(limit) {
        println!("1 / ({}) = {}", x.clone(), x.reciprocal());
    }
}

fn demo_gaussian_rational_reciprocal_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen_var_3().get(gm, config).take(limit) {
        println!("1 / (&{}) = {}", x, (&x).reciprocal());
    }
}

fn demo_gaussian_rational_reciprocal_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in gaussian_rational_gen_var_3().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.reciprocal_assign();
        println!("x := {old_x}; x.reciprocal_assign(); x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_reciprocal_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.reciprocal()",
        BenchmarkType::Algorithms,
        gaussian_rational_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                no_out!((&x).reciprocal());
            }),
            ("using conjugate and abs_squared", &mut |x| {
                no_out!((&x).conjugate() * GaussianRational::from((&x).abs_squared().reciprocal()));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_reciprocal_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.reciprocal()",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.reciprocal()", &mut |x| {
                no_out!(x.reciprocal());
            }),
            ("(&GaussianRational).reciprocal()", &mut |x| {
                no_out!((&x).reciprocal());
            }),
        ],
    );
}
