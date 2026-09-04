// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Content, ContentAndPrimitivePart, PrimitivePart};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_content_and_primitive_part);
    register_demo!(runner, demo_gaussian_integer_content_and_primitive_part_ref);
    register_demo!(runner, demo_gaussian_integer_content);
    register_demo!(runner, demo_gaussian_integer_content_ref);
    register_demo!(runner, demo_gaussian_integer_primitive_part);
    register_demo!(runner, demo_gaussian_integer_primitive_part_ref);

    register_bench!(
        runner,
        benchmark_gaussian_integer_content_and_primitive_part_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_gaussian_integer_content_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_gaussian_integer_primitive_part_evaluation_strategy
    );
}

fn demo_gaussian_integer_content_and_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (content, primitive) = x.content_and_primitive_part();
        println!("({x_old}).content_and_primitive_part() = ({content}, {primitive})");
    }
}

fn demo_gaussian_integer_content_and_primitive_part_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let (content, primitive) = (&x).content_and_primitive_part();
        println!("(&{x}).content_and_primitive_part() = ({content}, {primitive})");
    }
}

fn demo_gaussian_integer_content(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({x_old}).content() = {}", x.content());
    }
}

fn demo_gaussian_integer_content_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("(&{x}).content() = {}", (&x).content());
    }
}

fn demo_gaussian_integer_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({x_old}).primitive_part() = {}", x.primitive_part());
    }
}

fn demo_gaussian_integer_primitive_part_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("(&{x}).primitive_part() = {}", (&x).primitive_part());
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_content_and_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.content_and_primitive_part()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.content_and_primitive_part()", &mut |x| {
                no_out!(x.content_and_primitive_part());
            }),
            (
                "(&GaussianInteger).content_and_primitive_part()",
                &mut |x| {
                    no_out!((&x).content_and_primitive_part());
                },
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_content_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.content()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.content()", &mut |x| {
                no_out!(x.content());
            }),
            ("(&GaussianInteger).content()", &mut |x| {
                no_out!((&x).content());
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.primitive_part()",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [
            ("GaussianInteger.primitive_part()", &mut |x| {
                no_out!(x.primitive_part());
            }),
            ("(&GaussianInteger).primitive_part()", &mut |x| {
                no_out!((&x).primitive_part());
            }),
        ],
    );
}
