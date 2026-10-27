// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModPowerOf2CharacteristicPolynomial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::unsigned_matrix::arithmetic::{
    mod_power_of_2_characteristic_polynomial::*,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_matrix_mod_power_of_2_characteristic_polynomial
    );
    register_demo!(
        runner,
        demo_natural_matrix_mod_power_of_2_characteristic_polynomial_ref
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_characteristic_polynomial_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_characteristic_polynomial_algorithms
    );
}

fn demo_natural_matrix_mod_power_of_2_characteristic_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.is_square())
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_characteristic_polynomial({pow}) = {}",
            a.mod_power_of_2_characteristic_polynomial(pow)
        );
    }
}

fn demo_natural_matrix_mod_power_of_2_characteristic_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, pow) in natural_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.is_square())
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_characteristic_polynomial({pow}) = {}",
            (&a).mod_power_of_2_characteristic_polynomial(pow)
        );
    }
}

fn benchmark_natural_matrix_mod_power_of_2_characteristic_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_characteristic_polynomial(pow)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, _)| a.is_square()),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_power_of_2_characteristic_polynomial(pow)",
                &mut |(a, pow)| {
                    no_out!(a.mod_power_of_2_characteristic_polynomial(pow));
                },
            ),
            (
                "(&NaturalMatrix).mod_power_of_2_characteristic_polynomial(pow)",
                &mut |(a, pow)| {
                    no_out!((&a).mod_power_of_2_characteristic_polynomial(pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_power_of_2_characteristic_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_power_of_2_characteristic_polynomial(pow)",
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, _)| a.is_square()),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("Hessenberg", &mut |(a, pow)| {
                no_out!(a.mod_power_of_2_characteristic_polynomial(pow));
            }),
            ("Berkowitz", &mut |(a, pow)| {
                no_out!(NaturalPolynomial::from_coefficients_asc(
                    mod_power_of_2_characteristic_polynomial_berkowitz(
                        a.elements_ref(),
                        usize::exact_from(a.height()),
                        pow
                    )
                ));
            }),
        ],
    );
}
