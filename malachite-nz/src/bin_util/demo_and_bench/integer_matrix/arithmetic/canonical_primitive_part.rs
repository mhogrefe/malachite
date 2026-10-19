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
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_canonical_primitive_part);
    register_demo!(
        runner,
        demo_integer_matrix_content_and_canonical_primitive_part
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_canonical_primitive_part_evaluation_strategy
    );
}

fn demo_integer_matrix_canonical_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!(
            "({a}).canonical_primitive_part() = {}",
            (&a).canonical_primitive_part()
        );
    }
}

fn demo_integer_matrix_content_and_canonical_primitive_part(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let (content, primitive_part) = (&a).content_and_canonical_primitive_part();
        println!("({a}).content_and_canonical_primitive_part() = ({content}, {primitive_part})");
    }
}

fn benchmark_integer_matrix_canonical_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.canonical_primitive_part()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.canonical_primitive_part()", &mut |a| {
                no_out!(a.canonical_primitive_part());
            }),
            ("(&IntegerMatrix).canonical_primitive_part()", &mut |a| {
                no_out!((&a).canonical_primitive_part());
            }),
            (
                "IntegerMatrix.canonical_primitive_part_assign()",
                &mut |mut a| {
                    a.canonical_primitive_part_assign();
                },
            ),
            (
                "IntegerMatrix.content_and_canonical_primitive_part()",
                &mut |a| {
                    no_out!(a.content_and_canonical_primitive_part());
                },
            ),
        ],
    );
}
