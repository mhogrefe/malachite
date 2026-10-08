// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, ContentAndCanonicalPrimitivePart,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_gaussian_rational_content_and_canonical_primitive_part
    );
    register_demo!(
        runner,
        demo_gaussian_rational_content_and_canonical_primitive_part_ref
    );
    register_demo!(runner, demo_gaussian_rational_canonical_primitive_part);
    register_demo!(runner, demo_gaussian_rational_canonical_primitive_part_ref);

    register_bench!(
        runner,
        benchmark_gaussian_rational_content_and_canonical_primitive_part_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_gaussian_rational_canonical_primitive_part_evaluation_strategy
    );
}

fn demo_gaussian_rational_content_and_canonical_primitive_part(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (content, primitive) = x.content_and_canonical_primitive_part();
        println!("({x_old}).content_and_canonical_primitive_part() = ({content}, {primitive})");
    }
}

fn demo_gaussian_rational_content_and_canonical_primitive_part_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        let (content, primitive) = (&x).content_and_canonical_primitive_part();
        println!("(&{x}).content_and_canonical_primitive_part() = ({content}, {primitive})");
    }
}

fn demo_gaussian_rational_canonical_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({x_old}).canonical_primitive_part() = {}",
            x.canonical_primitive_part()
        );
    }
}

fn demo_gaussian_rational_canonical_primitive_part_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!(
            "(&{x}).canonical_primitive_part() = {}",
            (&x).canonical_primitive_part()
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_content_and_canonical_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.content_and_canonical_primitive_part()",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            (
                "GaussianRational.content_and_canonical_primitive_part()",
                &mut |x| {
                    no_out!(x.content_and_canonical_primitive_part());
                },
            ),
            (
                "(&GaussianRational).content_and_canonical_primitive_part()",
                &mut |x| {
                    no_out!((&x).content_and_canonical_primitive_part());
                },
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_canonical_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.canonical_primitive_part()",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.canonical_primitive_part()", &mut |x| {
                no_out!(x.canonical_primitive_part());
            }),
            ("(&GaussianRational).canonical_primitive_part()", &mut |x| {
                no_out!((&x).canonical_primitive_part());
            }),
        ],
    );
}
