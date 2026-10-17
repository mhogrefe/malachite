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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_from_elements);
    register_demo!(runner, demo_natural_matrix_from_owned_elements);
    register_bench!(
        runner,
        benchmark_natural_matrix_from_elements_evaluation_strategy
    );
}

fn demo_natural_matrix_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        let xs = m.elements.clone();
        println!(
            "NaturalMatrix::from_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            NaturalMatrix::from_elements(m.height(), m.width(), &xs)
        );
    }
}

fn demo_natural_matrix_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        let xs = m.elements.clone();
        println!(
            "NaturalMatrix::from_owned_elements({}, {}, {:?}) = {}",
            m.height(),
            m.width(),
            xs,
            NaturalMatrix::from_owned_elements(m.height(), m.width(), xs.clone())
        );
    }
}

fn benchmark_natural_matrix_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::from_elements(usize, usize, &[Natural])",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("m"),
        &mut [
            (
                "NaturalMatrix::from_elements(usize, usize, &[Natural])",
                &mut |m| {
                    no_out!(NaturalMatrix::from_elements(
                        m.height(),
                        m.width(),
                        &m.elements
                    ));
                },
            ),
            (
                "NaturalMatrix::from_owned_elements(usize, usize, Vec<Natural>)",
                &mut |m| {
                    let (height, width) = (m.height(), m.width());
                    no_out!(NaturalMatrix::from_owned_elements(
                        height, width, m.elements
                    ));
                },
            ),
        ],
    );
}
