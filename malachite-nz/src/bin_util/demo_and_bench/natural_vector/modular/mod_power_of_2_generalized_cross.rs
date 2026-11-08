// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::modular::traits::ModPowerOf2GeneralizedCross;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::modular::mod_power_of_2_generalized_cross::*;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_9;
use malachite_nz::test_util::natural_vector::modular::mod_power_of_2_generalized_cross::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_generalized_cross);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_generalized_cross_algorithms
    );
}

fn rows(a: &NaturalMatrix) -> Vec<NaturalVector> {
    a.rows()
        .map(|row| NaturalVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_natural_vector_mod_power_of_2_generalized_cross(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "mod_power_of_2_generalized_cross(rows of {a}, {pow}) = {}",
            NaturalVector::mod_power_of_2_generalized_cross(&rows(&a), pow)
        );
    }
}

fn benchmark_natural_vector_mod_power_of_2_generalized_cross_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::mod_power_of_2_generalized_cross",
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_pair_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("cofactors", &mut |(a, pow)| {
                no_out!(mod_power_of_2_generalized_cross_by_cofactors(
                    &rows(&a),
                    pow
                ));
            }),
            ("integers", &mut |(a, pow)| {
                no_out!(mod_power_of_2_generalized_cross_by_integers(&rows(&a), pow));
            }),
            ("determinants", &mut |(a, pow)| {
                no_out!(mod_power_of_2_generalized_cross_by_determinants(
                    &rows(&a),
                    pow
                ));
            }),
        ],
    );
}
