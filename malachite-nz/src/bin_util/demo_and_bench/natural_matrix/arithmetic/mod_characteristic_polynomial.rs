// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_3, natural_matrix_natural_pair_gen_var_4,
    natural_matrix_natural_pair_gen_var_6,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_characteristic_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_characteristic_polynomial);
    register_demo!(
        runner,
        demo_natural_matrix_mod_characteristic_polynomial_ref
    );
    register_demo!(
        runner,
        demo_natural_matrix_mod_characteristic_polynomial_large_moduli
    );
    register_demo!(
        runner,
        demo_natural_matrix_mod_characteristic_polynomial_large_dimensions
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_characteristic_polynomial_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_characteristic_polynomial_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_characteristic_polynomial_prime_algorithms
    );
}

fn demo_natural_matrix_mod_characteristic_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m_old = m.clone();
        println!(
            "{a_old}.mod_characteristic_polynomial({m_old}) = {}",
            a.mod_characteristic_polynomial(m)
        );
    }
}

fn demo_natural_matrix_mod_characteristic_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_characteristic_polynomial(&{m}) = {}",
            (&a).mod_characteristic_polynomial(&m)
        );
    }
}

fn demo_natural_matrix_mod_characteristic_polynomial_large_moduli(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in natural_matrix_natural_pair_gen_var_4()
        .get(gm, config)
        .filter(|(a, _)| a.is_square())
        .take(limit)
    {
        println!(
            "(&{a}).mod_characteristic_polynomial(&{m}) = {}",
            (&a).mod_characteristic_polynomial(&m)
        );
    }
}

fn demo_natural_matrix_mod_characteristic_polynomial_large_dimensions(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", 20);
    for (a, m) in natural_matrix_natural_pair_gen_var_6()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_characteristic_polynomial(&{m}) = {}",
            (&a).mod_characteristic_polynomial(&m)
        );
    }
}

fn benchmark_natural_matrix_mod_characteristic_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_characteristic_polynomial(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_characteristic_polynomial(Natural)",
                &mut |(a, m)| {
                    no_out!(a.mod_characteristic_polynomial(m));
                },
            ),
            (
                "NaturalMatrix.mod_characteristic_polynomial(&Natural)",
                &mut |(a, m)| {
                    no_out!(a.mod_characteristic_polynomial(&m));
                },
            ),
            (
                "(&NaturalMatrix).mod_characteristic_polynomial(Natural)",
                &mut |(a, m)| {
                    no_out!((&a).mod_characteristic_polynomial(m));
                },
            ),
            (
                "(&NaturalMatrix).mod_characteristic_polynomial(&Natural)",
                &mut |(a, m)| {
                    no_out!((&a).mod_characteristic_polynomial(&m));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_characteristic_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_characteristic_polynomial(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_characteristic_polynomial(&m));
            }),
            ("Berkowitz", &mut |(a, m)| {
                no_out!(natural_matrix_mod_characteristic_polynomial_berkowitz(
                    &a, &m
                ));
            }),
            ("Hessenberg", &mut |(a, m)| {
                no_out!(natural_matrix_mod_characteristic_polynomial_hessenberg(
                    &a, &m
                ));
            }),
        ],
    );
}

fn benchmark_natural_matrix_mod_characteristic_polynomial_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_characteristic_polynomial(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_characteristic_polynomial(&m));
            }),
            ("Berkowitz", &mut |(a, m)| {
                no_out!(natural_matrix_mod_characteristic_polynomial_berkowitz(
                    &a, &m
                ));
            }),
            ("Hessenberg", &mut |(a, m)| {
                no_out!(natural_matrix_mod_characteristic_polynomial_hessenberg(
                    &a, &m
                ));
            }),
            ("Danilevsky", &mut |(a, m)| {
                no_out!(natural_matrix_mod_characteristic_polynomial_danilevsky(
                    &a, &m
                ));
            }),
        ],
    );
}
