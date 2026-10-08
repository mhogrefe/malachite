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
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_content);
    register_demo!(runner, demo_rational_vector_primitive_part);
    register_demo!(runner, demo_rational_vector_content_and_primitive_part);
    register_bench!(
        runner,
        benchmark_rational_vector_content_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_vector_primitive_part_evaluation_strategy
    );
}

fn demo_rational_vector_content(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("({v}).content() = {}", (&v).content());
    }
}

fn demo_rational_vector_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("({v}).primitive_part() = {}", (&v).primitive_part());
    }
}

fn demo_rational_vector_content_and_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        let (content, primitive_part) = (&v).content_and_primitive_part();
        println!("({v}).content_and_primitive_part() = ({content}, {primitive_part})");
    }
}

fn benchmark_rational_vector_content_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.content()",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector.content()", &mut |v| {
                no_out!(v.content());
            }),
            ("(&RationalVector).content()", &mut |v| {
                no_out!((&v).content());
            }),
        ],
    );
}

fn benchmark_rational_vector_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.primitive_part()",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector.primitive_part()", &mut |v| {
                no_out!(v.primitive_part());
            }),
            ("(&RationalVector).primitive_part()", &mut |v| {
                no_out!((&v).primitive_part());
            }),
            ("RationalVector.content_and_primitive_part()", &mut |v| {
                no_out!(v.content_and_primitive_part());
            }),
        ],
    );
}
