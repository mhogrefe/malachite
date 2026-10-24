// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Determinant;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::determinant::*;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_gen_var_2, integer_matrix_pair_gen_var_7,
};
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;
use malachite_nz::test_util::integer_matrix::arithmetic::solve::integer_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_determinant);
    register_demo!(runner, demo_integer_matrix_determinant_ref);
    register_demo!(runner, demo_integer_matrix_determinant_low_rank);
    register_demo!(runner, demo_integer_matrix_determinant_modular);
    register_demo!(runner, demo_integer_matrix_determinant_modular_low_rank);
    register_demo!(runner, demo_integer_matrix_determinant_modular_unproved);
    register_demo!(runner, demo_integer_matrix_determinant_modular_accelerated);
    register_demo!(
        runner,
        demo_integer_matrix_determinant_modular_accelerated_low_rank
    );
    register_demo!(runner, demo_integer_matrix_determinant_bareiss);
    register_demo!(runner, demo_integer_matrix_determinant_cofactor);
    register_bench!(runner, benchmark_integer_matrix_determinant_algorithms);
    register_bench!(
        runner,
        benchmark_integer_matrix_determinant_small_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_determinant_evaluation_strategy
    );
}

fn demo_integer_matrix_determinant(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("determinant({a_old}) = {}", a.determinant());
    }
}

fn demo_integer_matrix_determinant_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("(&{a}).determinant() = {}", (&a).determinant());
    }
}

// Square matrices of rank at most the width of a second matrix, when that is smaller than their
// dimension, so that many are singular.
fn demo_integer_matrix_determinant_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = integer_matrix_low_rank(&a, &b);
        println!("determinant({a}) = {}", (&a).determinant());
    }
}

fn demo_integer_matrix_determinant_modular(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!(
            "determinant_modular({a}, true) = {}",
            determinant_modular(&a, true)
        );
    }
}

fn demo_integer_matrix_determinant_modular_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = integer_matrix_low_rank(&a, &b);
        println!(
            "determinant_modular({a}, true) = {}",
            determinant_modular(&a, true)
        );
    }
}

fn demo_integer_matrix_determinant_modular_unproved(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!(
            "determinant_modular({a}, false) = {}",
            determinant_modular(&a, false)
        );
    }
}

fn demo_integer_matrix_determinant_modular_accelerated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!(
            "determinant_modular_accelerated({a}, true) = {}",
            determinant_modular_accelerated(&a, true)
        );
    }
}

fn demo_integer_matrix_determinant_modular_accelerated_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = integer_matrix_low_rank(&a, &b);
        println!(
            "determinant_modular_accelerated({a}, true) = {}",
            determinant_modular_accelerated(&a, true)
        );
    }
}

fn benchmark_integer_matrix_determinant_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.determinant()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.determinant()", &mut |a| {
                no_out!(a.determinant());
            }),
            ("(&IntegerMatrix).determinant()", &mut |a| {
                no_out!((&a).determinant());
            }),
        ],
    );
}

fn demo_integer_matrix_determinant_bareiss(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("determinant_bareiss({a}) = {}", determinant_bareiss(&a));
    }
}

fn demo_integer_matrix_determinant_cofactor(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_2().get(gm, config).take(limit) {
        println!("determinant_cofactor({a}) = {}", determinant_cofactor(&a));
    }
}

fn benchmark_integer_matrix_determinant_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "determinant(&IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("Bareiss", &mut |a| {
                no_out!(determinant_bareiss(&a));
            }),
            ("Bareiss, Integer arithmetic only", &mut |a| {
                no_out!(integer_matrix_determinant_bareiss_general(&a));
            }),
            ("multimodular", &mut |a| {
                no_out!(determinant_modular(&a, true));
            }),
            ("multimodular with a divisor", &mut |a| {
                no_out!(determinant_modular_accelerated(&a, true));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_determinant_naive(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_determinant_small_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "determinant(&IntegerMatrix) for dimensions up to 4",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("cofactor", &mut |a| {
                no_out!(determinant_cofactor(&a));
            }),
            ("Bareiss", &mut |a| {
                no_out!(determinant_bareiss(&a));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_determinant_naive(&a));
            }),
        ],
    );
}
