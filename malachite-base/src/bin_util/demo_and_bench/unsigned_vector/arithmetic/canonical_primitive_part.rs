// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
};
use malachite_base::test_util::bench::bucketers::unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_canonical_primitive_part);
    register_demo!(
        runner,
        demo_unsigned_vector_content_and_canonical_primitive_part
    );
    register_bench!(
        runner,
        benchmark_unsigned_vector_canonical_primitive_part_evaluation_strategy
    );
}

fn demo_unsigned_vector_canonical_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!(
            "({v}).canonical_primitive_part() = {}",
            (&v).canonical_primitive_part()
        );
    }
}

fn demo_unsigned_vector_content_and_canonical_primitive_part(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let (content, canonical_primitive_part) = (&v).content_and_canonical_primitive_part();
        println!(
            "({v}).content_and_canonical_primitive_part() = ({content}, {canonical_primitive_part})"
        );
    }
}

fn benchmark_unsigned_vector_canonical_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.canonical_primitive_part()",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.canonical_primitive_part()", &mut |v| {
                no_out!(v.canonical_primitive_part());
            }),
            (
                "(&UnsignedVector<u64>).canonical_primitive_part()",
                &mut |v| {
                    no_out!((&v).canonical_primitive_part());
                },
            ),
            (
                "UnsignedVector<u64>.canonical_primitive_part_assign()",
                &mut |mut v| {
                    v.canonical_primitive_part_assign();
                },
            ),
            (
                "UnsignedVector<u64>.content_and_canonical_primitive_part()",
                &mut |v| {
                    no_out!(v.content_and_canonical_primitive_part());
                },
            ),
        ],
    );
}
