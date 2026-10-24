// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::HermiteNormalFormAndTransform;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::hermite_normal_form_and_transform::*;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_integer_matrix_hermite_normal_form_and_transform
    );
    register_demo!(
        runner,
        demo_integer_matrix_hermite_normal_form_and_transform_val
    );
    register_demo!(
        runner,
        demo_integer_matrix_hermite_normal_form_and_transform_low_rank
    );
    register_demo!(runner, demo_integer_matrix_hnf_minors_transform);
    register_demo!(runner, demo_integer_matrix_hnf_transform_naive);
    register_bench!(
        runner,
        benchmark_integer_matrix_hermite_normal_form_and_transform_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_hermite_normal_form_and_transform_algorithms
    );
}

fn demo_integer_matrix_hermite_normal_form_and_transform(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let (h, u) = (&a).hermite_normal_form_and_transform();
        println!("hermite_normal_form_and_transform({a}) = ({h}, {u})");
    }
}

fn demo_integer_matrix_hermite_normal_form_and_transform_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let (h, u) = a.hermite_normal_form_and_transform();
        println!("{a_old}.hermite_normal_form_and_transform() = ({h}, {u})");
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_hermite_normal_form_and_transform_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        let (h, u) = (&a).hermite_normal_form_and_transform();
        println!("hermite_normal_form_and_transform({a}) = ({h}, {u})");
    }
}

fn demo_integer_matrix_hnf_minors_transform(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_3().get(gm, config).take(limit) {
        let (h, u) = hnf_minors_transform(&a);
        println!("hnf_minors_transform({a}) = ({h}, {u})");
    }
}

fn demo_integer_matrix_hnf_transform_naive(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let (h, u) = hnf_transform_naive(&a);
        println!("hnf_transform_naive({a}) = ({h}, {u})");
    }
}

fn benchmark_integer_matrix_hermite_normal_form_and_transform_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.hermite_normal_form_and_transform()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            (
                "IntegerMatrix.hermite_normal_form_and_transform()",
                &mut |a| {
                    no_out!(a.hermite_normal_form_and_transform());
                },
            ),
            (
                "(&IntegerMatrix).hermite_normal_form_and_transform()",
                &mut |a| {
                    no_out!((&a).hermite_normal_form_and_transform());
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_hermite_normal_form_and_transform_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.hermite_normal_form_and_transform()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.hermite_normal_form_and_transform());
            }),
            ("minors", &mut |a| {
                no_out!(hnf_minors_transform(&a));
            }),
            ("naive", &mut |a| {
                no_out!(hnf_transform_naive(&a));
            }),
        ],
    );
}
