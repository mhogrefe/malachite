// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_from_rows);
    register_demo!(runner, demo_rational_matrix_from_owned_rows);
    register_bench!(
        runner,
        benchmark_rational_matrix_from_rows_evaluation_strategy
    );
}

fn rows_of(m: &RationalMatrix) -> Vec<RationalVector> {
    (0..m.height())
        .map(|i| {
            RationalVector::from_elements(
                &m.elements_ref()
                    [usize::exact_from(i * m.width())..usize::exact_from((i + 1) * m.width())],
            )
        })
        .collect()
}

fn demo_rational_matrix_from_rows(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        let vs = rows_of(&m);
        println!(
            "RationalMatrix::from_rows({:?}) = {}",
            vs,
            RationalMatrix::from_rows(&vs)
        );
    }
}

fn demo_rational_matrix_from_owned_rows(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        let vs = rows_of(&m);
        println!(
            "RationalMatrix::from_owned_rows({:?}) = {}",
            vs,
            RationalMatrix::from_owned_rows(vs.clone())
        );
    }
}

fn benchmark_rational_matrix_from_rows_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix::from_rows(&[RationalVector])",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen()
            .get(gm, config)
            .map(|m| (rows_of(&m), m)),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(_, m)| {
                usize::exact_from(
                    m.elements_ref()
                        .iter()
                        .map(SignificantBits::significant_bits)
                        .sum::<u64>(),
                )
            },
            bucketing_label: "m's total element bits".to_string(),
        },
        &mut [
            (
                "RationalMatrix::from_rows(&[RationalVector])",
                &mut |(vs, _)| {
                    no_out!(RationalMatrix::from_rows(&vs));
                },
            ),
            (
                "RationalMatrix::from_owned_rows(Vec<RationalVector>)",
                &mut |(vs, _)| {
                    no_out!(RationalMatrix::from_owned_rows(vs));
                },
            ),
        ],
    );
}
