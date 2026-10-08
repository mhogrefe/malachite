// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};
use malachite_base::test_util::bench::bucketers::unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_content);
    register_demo!(runner, demo_unsigned_vector_primitive_part);
    register_demo!(runner, demo_unsigned_vector_content_and_primitive_part);
    register_bench!(
        runner,
        benchmark_unsigned_vector_content_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_vector_primitive_part_evaluation_strategy
    );
}

fn demo_unsigned_vector_content(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("({v}).content() = {}", (&v).content());
    }
}

fn demo_unsigned_vector_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("({v}).primitive_part() = {}", (&v).primitive_part());
    }
}

fn demo_unsigned_vector_content_and_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let (content, primitive_part) = (&v).content_and_primitive_part();
        println!("({v}).content_and_primitive_part() = ({content}, {primitive_part})");
    }
}

fn benchmark_unsigned_vector_content_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.content()",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.content()", &mut |v| {
                no_out!(v.content());
            }),
            ("(&UnsignedVector<u64>).content()", &mut |v| {
                no_out!((&v).content());
            }),
        ],
    );
}

fn benchmark_unsigned_vector_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.primitive_part()",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.primitive_part()", &mut |v| {
                no_out!(v.primitive_part());
            }),
            ("(&UnsignedVector<u64>).primitive_part()", &mut |v| {
                no_out!((&v).primitive_part());
            }),
            (
                "UnsignedVector<u64>.primitive_part_assign()",
                &mut |mut v| {
                    v.primitive_part_assign();
                },
            ),
            (
                "UnsignedVector<u64>.content_and_primitive_part()",
                &mut |v| {
                    no_out!(v.content_and_primitive_part());
                },
            ),
        ],
    );
}
