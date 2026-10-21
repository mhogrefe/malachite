// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign, ModPowerOf2};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_mul_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_mul_matrix);
    register_demo!(runner, demo_unsigned_matrix_mod_mul_matrix_assign);
    register_demo!(runner, demo_unsigned_matrix_mod_mul_matrix_large_dimensions);
    register_demo!(runner, demo_unsigned_matrix_mod_mul_matrix_small_moduli);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_mul_matrix_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_matrix_mod_mul_matrix_algorithms);
}

fn demo_unsigned_matrix_mod_mul_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("{a}.mod_mul({b}, {m}) = {}", (&a).mod_mul(&b, m));
    }
}

fn demo_unsigned_matrix_mod_mul_matrix_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_mul_assign(&b, m);
        println!("a := {a_old}; a.mod_mul_assign({b}, {m}); a = {a}");
    }
}

// Products whose every dimension is between 200 and 600, so that they reach Strassen's algorithm,
// whose cutoff is 200 or 400 depending on the modulus. The lines are long.
fn demo_unsigned_matrix_mod_mul_matrix_large_dimensions(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", 300);
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3()
        .get(gm, &config)
        .filter(|(a, b, _)| {
            [a.height(), a.width(), b.width()]
                .iter()
                .all(|d| (200..=600).contains(d))
        })
        .take(limit)
    {
        println!("{a}.mod_mul({b}, {m}) = {}", (&a).mod_mul(&b, m));
    }
}

// Moduli below 2^10 at dimensions above 10, where the classical algorithm packs several elements
// into a word.
fn demo_unsigned_matrix_mod_mul_matrix_small_moduli(gm: GenMode, config: &GenConfig, limit: usize) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", 20);
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        let small_m = m.mod_power_of_2(10).max(2);
        let a = a % small_m;
        let b = b % small_m;
        println!(
            "{a}.mod_mul({b}, {small_m}) = {}",
            (&a).mod_mul(&b, small_m)
        );
    }
}

fn benchmark_unsigned_matrix_mod_mul_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_mul(UnsignedMatrix<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_mul(UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_mul(b, m));
                },
            ),
            (
                "UnsignedMatrix.mod_mul(&UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_mul(&b, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_mul(UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_mul(b, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_mul(&UnsignedMatrix, u64)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_mul(&b, m));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_mul_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_mul(UnsignedMatrix<u64>, u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, m)| {
                no_out!((&a).mod_mul(&b, m));
            }),
            ("classical", &mut |(a, b, m)| {
                no_out!(unsigned_matrix_mod_mul_matrix_classical(&a, &b, m));
            }),
            ("Strassen", &mut |(a, b, m)| {
                no_out!(unsigned_matrix_mod_mul_matrix_strassen(&a, &b, m));
            }),
            ("naive", &mut |(a, b, m)| {
                no_out!(unsigned_matrix_mod_mul_matrix_naive(&a, &b, m));
            }),
        ],
    );
}
