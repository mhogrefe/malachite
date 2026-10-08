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
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_content);
    register_demo!(runner, demo_natural_vector_primitive_part);
    register_demo!(runner, demo_natural_vector_content_and_primitive_part);
    register_bench!(runner, benchmark_natural_vector_content_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_natural_vector_primitive_part_evaluation_strategy
    );
}

fn demo_natural_vector_content(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!("({v}).content() = {}", (&v).content());
    }
}

fn demo_natural_vector_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!("({v}).primitive_part() = {}", (&v).primitive_part());
    }
}

fn demo_natural_vector_content_and_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        let (content, primitive_part) = (&v).content_and_primitive_part();
        println!("({v}).content_and_primitive_part() = ({content}, {primitive_part})");
    }
}

fn benchmark_natural_vector_content_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.content()",
        BenchmarkType::EvaluationStrategy,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.content()", &mut |v| {
                no_out!(v.content());
            }),
            ("(&NaturalVector).content()", &mut |v| {
                no_out!((&v).content());
            }),
        ],
    );
}

fn benchmark_natural_vector_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.primitive_part()",
        BenchmarkType::EvaluationStrategy,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.primitive_part()", &mut |v| {
                no_out!(v.primitive_part());
            }),
            ("(&NaturalVector).primitive_part()", &mut |v| {
                no_out!((&v).primitive_part());
            }),
            ("NaturalVector.primitive_part_assign()", &mut |mut v| {
                v.primitive_part_assign();
            }),
            ("NaturalVector.content_and_primitive_part()", &mut |v| {
                no_out!(v.content_and_primitive_part());
            }),
        ],
    );
}
