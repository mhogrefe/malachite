// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_content);
    register_demo!(runner, demo_integer_polynomial_primitive_part);
    register_demo!(runner, demo_integer_polynomial_content_and_primitive_part);
    register_bench!(
        runner,
        benchmark_integer_polynomial_content_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_primitive_part_evaluation_strategy
    );
}

fn demo_integer_polynomial_content(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).content() = {}", (&p).content());
    }
}

fn demo_integer_polynomial_primitive_part(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).primitive_part() = {}", (&p).primitive_part());
    }
}

fn demo_integer_polynomial_content_and_primitive_part(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        let (content, primitive_part) = (&p).content_and_primitive_part();
        println!("({p}).content_and_primitive_part() = ({content}, {primitive_part})");
    }
}

fn benchmark_integer_polynomial_content_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.content()",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.content()", &mut |p| {
                no_out!(p.content());
            }),
            ("(&IntegerPolynomial).content()", &mut |p| {
                no_out!((&p).content());
            }),
        ],
    );
}

fn benchmark_integer_polynomial_primitive_part_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.primitive_part()",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.primitive_part()", &mut |p| {
                no_out!(p.primitive_part());
            }),
            ("(&IntegerPolynomial).primitive_part()", &mut |p| {
                no_out!((&p).primitive_part());
            }),
            ("IntegerPolynomial.primitive_part_assign()", &mut |mut p| {
                p.primitive_part_assign();
            }),
            ("IntegerPolynomial.content_and_primitive_part()", &mut |p| {
                no_out!(p.content_and_primitive_part());
            }),
        ],
    );
}
