// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Determinant, SmithNormalForm, SmithNormalFormAssign, UnsignedAbs,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::smith_normal_form::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_smith_normal_form);
    register_demo!(runner, demo_integer_matrix_smith_normal_form_val);
    register_demo!(runner, demo_integer_matrix_smith_normal_form_assign);
    register_demo!(runner, demo_integer_matrix_smith_normal_form_low_rank);
    register_demo!(runner, demo_integer_matrix_snf_diagonal);
    register_demo!(runner, demo_integer_matrix_snf_iterative_hermite);
    register_demo!(runner, demo_integer_matrix_snf_iterative_hermite_low_rank);
    register_demo!(runner, demo_integer_matrix_snf_kannan_bachem);
    register_demo!(runner, demo_integer_matrix_snf_kannan_bachem_low_rank);
    register_demo!(runner, demo_integer_matrix_snf_iliopoulos);
    register_bench!(
        runner,
        benchmark_integer_matrix_smith_normal_form_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_smith_normal_form_algorithms
    );
}

// Whether `a`, which has full column rank, is square, and hence nonsingular.
fn nonsingular(a: &IntegerMatrix) -> bool {
    a.height() == a.width()
}

fn demo_integer_matrix_smith_normal_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("smith_normal_form({a}) = {}", (&a).smith_normal_form());
    }
}

fn demo_integer_matrix_smith_normal_form_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.smith_normal_form() = {}", a.smith_normal_form());
    }
}

fn demo_integer_matrix_smith_normal_form_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.smith_normal_form_assign();
        println!("a := {a_old}; a.smith_normal_form_assign(); a = {a}");
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_smith_normal_form_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("smith_normal_form({a}) = {}", (&a).smith_normal_form());
    }
}

// Only the diagonal elements are read; the others are copied, as by FLINT.
fn demo_integer_matrix_snf_diagonal(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("snf_diagonal({a}) = {}", snf_diagonal(&a));
    }
}

fn demo_integer_matrix_snf_iterative_hermite(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("snf_iterative_hermite({a}) = {}", snf_iterative_hermite(&a));
    }
}

fn demo_integer_matrix_snf_iterative_hermite_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("snf_iterative_hermite({a}) = {}", snf_iterative_hermite(&a));
    }
}

fn demo_integer_matrix_snf_kannan_bachem(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("snf_kannan_bachem({a}) = {}", snf_kannan_bachem(&a));
    }
}

fn demo_integer_matrix_snf_kannan_bachem_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("snf_kannan_bachem({a}) = {}", snf_kannan_bachem(&a));
    }
}

// Each nonsingular matrix twice: with the absolute value of its determinant, which is a multiple of
// its largest elementary divisor, and with a multiple of that.
fn demo_integer_matrix_snf_iliopoulos(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_3()
        .get(gm, config)
        .filter(nonsingular)
        .take(limit)
    {
        let d = (&a).determinant().unsigned_abs();
        println!("snf_iliopoulos({a}, {d}) = {}", snf_iliopoulos(&a, &d));
        let d = d * Natural::from(6u32);
        println!("snf_iliopoulos({a}, {d}) = {}", snf_iliopoulos(&a, &d));
    }
}

fn benchmark_integer_matrix_smith_normal_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.smith_normal_form()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.smith_normal_form()", &mut |a| {
                no_out!(a.smith_normal_form());
            }),
            ("(&IntegerMatrix).smith_normal_form()", &mut |a| {
                no_out!((&a).smith_normal_form());
            }),
            ("IntegerMatrix.smith_normal_form_assign()", &mut |mut a| {
                a.smith_normal_form_assign();
            }),
        ],
    );
}

fn benchmark_integer_matrix_smith_normal_form_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.smith_normal_form()",
        BenchmarkType::Algorithms,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.smith_normal_form());
            }),
            ("iterative Hermite", &mut |a| {
                no_out!(snf_iterative_hermite(&a));
            }),
            ("Kannan-Bachem", &mut |a| {
                no_out!(snf_kannan_bachem(&a));
            }),
        ],
    );
}
