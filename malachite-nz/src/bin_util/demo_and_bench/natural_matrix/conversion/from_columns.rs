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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_from_columns);
    register_demo!(runner, demo_natural_matrix_from_owned_columns);
    register_bench!(
        runner,
        benchmark_natural_matrix_from_columns_evaluation_strategy
    );
}

fn columns_of(m: &NaturalMatrix) -> Vec<NaturalVector> {
    (0..m.width())
        .map(|j| {
            NaturalVector::from_owned_elements(
                (0..m.height())
                    .map(|i| m.elements[usize::exact_from(i * m.width() + j)].clone())
                    .collect(),
            )
        })
        .collect()
}

fn demo_natural_matrix_from_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        let vs = columns_of(&m);
        println!(
            "NaturalMatrix::from_columns({:?}) = {}",
            vs,
            NaturalMatrix::from_columns(&vs)
        );
    }
}

fn demo_natural_matrix_from_owned_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        let vs = columns_of(&m);
        println!(
            "NaturalMatrix::from_owned_columns({:?}) = {}",
            vs,
            NaturalMatrix::from_owned_columns(vs.clone())
        );
    }
}

fn benchmark_natural_matrix_from_columns_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::from_columns(&[NaturalVector])",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen()
            .get(gm, config)
            .map(|m| (columns_of(&m), m)),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(_, m)| {
                usize::exact_from(
                    m.elements
                        .iter()
                        .map(SignificantBits::significant_bits)
                        .sum::<u64>(),
                )
            },
            bucketing_label: "m's total element bits".to_string(),
        },
        &mut [
            (
                "NaturalMatrix::from_columns(&[NaturalVector])",
                &mut |(vs, _)| {
                    no_out!(NaturalMatrix::from_columns(&vs));
                },
            ),
            (
                "NaturalMatrix::from_owned_columns(Vec<NaturalVector>)",
                &mut |(vs, _)| {
                    no_out!(NaturalMatrix::from_owned_columns(vs));
                },
            ),
        ],
    );
}
