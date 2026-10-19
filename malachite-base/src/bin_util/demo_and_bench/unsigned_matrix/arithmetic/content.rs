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
use malachite_base::test_util::bench::bucketers::unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_content);
    register_demo!(runner, demo_unsigned_matrix_primitive_part);
    register_demo!(runner, demo_unsigned_matrix_content_and_primitive_part);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_content_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_primitive_part_evaluation_strategy
    );
}

fn demo_unsigned_matrix_content(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in unsigned_matrix_gen().get(gm, config).take(limit) {
        println!("({a}).content() = {}", (&a).content());
    }
}

fn demo_unsigned_matrix_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in unsigned_matrix_gen().get(gm, config).take(limit) {
        println!("({a}).primitive_part() = {}", (&a).primitive_part());
    }
}

fn demo_unsigned_matrix_content_and_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in unsigned_matrix_gen().get(gm, config).take(limit) {
        let (content, primitive_part) = (&a).content_and_primitive_part();
        println!("({a}).content_and_primitive_part() = ({content}, {primitive_part})");
    }
}

fn benchmark_unsigned_matrix_content_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.content()",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.content()", &mut |a| {
                no_out!(a.content());
            }),
            ("(&UnsignedMatrix).content()", &mut |a| {
                no_out!((&a).content());
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.primitive_part()",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix.primitive_part()", &mut |a| {
                no_out!(a.primitive_part());
            }),
            ("(&UnsignedMatrix).primitive_part()", &mut |a| {
                no_out!((&a).primitive_part());
            }),
            ("UnsignedMatrix.primitive_part_assign()", &mut |mut a| {
                a.primitive_part_assign();
            }),
            ("UnsignedMatrix.content_and_primitive_part()", &mut |a| {
                no_out!(a.content_and_primitive_part());
            }),
        ],
    );
}
