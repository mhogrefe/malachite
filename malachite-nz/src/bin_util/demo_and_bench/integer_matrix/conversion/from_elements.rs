// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_from_elements);
    register_demo!(runner, demo_integer_matrix_from_owned_elements);
    register_bench!(
        runner,
        benchmark_integer_matrix_from_elements_evaluation_strategy
    );
}

fn demo_integer_matrix_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        let xs = m.to_elements();
        println!(
            "IntegerMatrix::from_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            IntegerMatrix::from_elements(m.height(), m.width(), &xs)
        );
    }
}

fn demo_integer_matrix_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        let xs = m.to_elements();
        println!(
            "IntegerMatrix::from_owned_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            IntegerMatrix::from_owned_elements(m.height(), m.width(), xs.clone())
        );
    }
}

fn benchmark_integer_matrix_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::from_elements(usize, usize, &[Integer])",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("m"),
        &mut [
            (
                "IntegerMatrix::from_elements(usize, usize, &[Integer])",
                &mut |m| {
                    no_out!(IntegerMatrix::from_elements(
                        m.height(),
                        m.width(),
                        &m.elements_ref()
                    ));
                },
            ),
            (
                "IntegerMatrix::from_owned_elements(usize, usize, Vec<Integer>)",
                &mut |m| {
                    let (height, width) = (m.height(), m.width());
                    no_out!(IntegerMatrix::from_owned_elements(
                        height,
                        width,
                        m.into_elements()
                    ));
                },
            ),
        ],
    );
}
