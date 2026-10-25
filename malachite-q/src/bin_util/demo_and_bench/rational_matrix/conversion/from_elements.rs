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
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_from_elements);
    register_demo!(runner, demo_rational_matrix_from_owned_elements);
    register_bench!(
        runner,
        benchmark_rational_matrix_from_elements_evaluation_strategy
    );
}

fn demo_rational_matrix_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        let xs = m.to_elements();
        println!(
            "RationalMatrix::from_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            RationalMatrix::from_elements(m.height(), m.width(), &xs)
        );
    }
}

fn demo_rational_matrix_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        let xs = m.to_elements();
        println!(
            "RationalMatrix::from_owned_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            RationalMatrix::from_owned_elements(m.height(), m.width(), xs.clone())
        );
    }
}

fn benchmark_rational_matrix_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix::from_elements(usize, usize, &[Rational])",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("m"),
        &mut [
            (
                "RationalMatrix::from_elements(usize, usize, &[Rational])",
                &mut |m| {
                    no_out!(RationalMatrix::from_elements(
                        m.height(),
                        m.width(),
                        m.elements_ref()
                    ));
                },
            ),
            (
                "RationalMatrix::from_owned_elements(usize, usize, Vec<Rational>)",
                &mut |m| {
                    let (height, width) = (m.height(), m.width());
                    no_out!(RationalMatrix::from_owned_elements(
                        height,
                        width,
                        m.into_elements()
                    ));
                },
            ),
        ],
    );
}
