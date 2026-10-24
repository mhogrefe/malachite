// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ReducedRowEchelonForm, ReducedRowEchelonFormAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::reduced_row_echelon_form::{rref_fflu, rref_mul};
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_reduced_row_echelon_form);
    register_demo!(
        runner,
        demo_integer_matrix_reduced_row_echelon_form_low_rank
    );
    register_demo!(runner, demo_integer_matrix_rref_fflu);
    register_demo!(runner, demo_integer_matrix_rref_fflu_low_rank);
    register_demo!(runner, demo_integer_matrix_rref_mul);
    register_demo!(runner, demo_integer_matrix_rref_mul_low_rank);
    register_demo!(runner, demo_integer_matrix_reduced_row_echelon_form_val);
    register_demo!(runner, demo_integer_matrix_reduced_row_echelon_form_assign);
    register_bench!(
        runner,
        benchmark_integer_matrix_reduced_row_echelon_form_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_reduced_row_echelon_form_algorithms
    );
}

fn rref_fflu_of(a: &IntegerMatrix) -> (IntegerMatrix, Integer) {
    let mut a = a.clone();
    let den = rref_fflu(&mut a).1;
    (a, den)
}

fn rref_mul_of(a: &IntegerMatrix) -> (IntegerMatrix, Integer) {
    let (_, r, den) = rref_mul(a);
    (r, den)
}

fn demo_integer_matrix_reduced_row_echelon_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let (r, den) = (&a).reduced_row_echelon_form();
        println!("reduced_row_echelon_form({a}) = ({r}, {den})");
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_reduced_row_echelon_form_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        let (r, den) = (&a).reduced_row_echelon_form();
        println!("reduced_row_echelon_form({a}) = ({r}, {den})");
    }
}

fn demo_integer_matrix_rref_fflu(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let (r, den) = rref_fflu_of(&a);
        println!("rref_fflu({a}) = ({r}, {den})");
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_rref_fflu_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        let (r, den) = rref_fflu_of(&a);
        println!("rref_fflu({a}) = ({r}, {den})");
    }
}

fn demo_integer_matrix_rref_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let (r, den) = rref_mul_of(&a);
        println!("rref_mul({a}) = ({r}, {den})");
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_rref_mul_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        let (r, den) = rref_mul_of(&a);
        println!("rref_mul({a}) = ({r}, {den})");
    }
}

fn demo_integer_matrix_reduced_row_echelon_form_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "{a_old}.reduced_row_echelon_form() = {:?}",
            a.reduced_row_echelon_form()
        );
    }
}

fn demo_integer_matrix_reduced_row_echelon_form_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let den = a.reduced_row_echelon_form_assign();
        println!("a := {a_old}; a.reduced_row_echelon_form_assign() = {den}; a = {a}");
    }
}

fn benchmark_integer_matrix_reduced_row_echelon_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.reduced_row_echelon_form()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.reduced_row_echelon_form()", &mut |a| {
                no_out!(a.reduced_row_echelon_form());
            }),
            ("(&IntegerMatrix).reduced_row_echelon_form()", &mut |a| {
                no_out!((&a).reduced_row_echelon_form());
            }),
            (
                "IntegerMatrix.reduced_row_echelon_form_assign()",
                &mut |mut a| {
                    no_out!(a.reduced_row_echelon_form_assign());
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_reduced_row_echelon_form_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.reduced_row_echelon_form()",
        BenchmarkType::Algorithms,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("fraction-free LU", &mut |a| {
                no_out!(rref_fflu_of(&a));
            }),
            ("multimodular", &mut |a| {
                no_out!(rref_mul_of(&a));
            }),
        ],
    );
}
