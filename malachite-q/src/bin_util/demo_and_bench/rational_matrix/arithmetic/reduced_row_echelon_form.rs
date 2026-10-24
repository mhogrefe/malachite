// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Nullspace, Rank, ReducedRowEchelonForm, ReducedRowEchelonFormAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::arithmetic::reduced_row_echelon_form::{
    rref_classical, rref_fraction_free,
};
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_3, rational_matrix_pair_gen_var_6,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_reduced_row_echelon_form);
    register_demo!(
        runner,
        demo_rational_matrix_reduced_row_echelon_form_low_rank
    );
    register_demo!(runner, demo_rational_matrix_rref_classical);
    register_demo!(runner, demo_rational_matrix_rref_classical_low_rank);
    register_demo!(runner, demo_rational_matrix_rref_fraction_free);
    register_demo!(runner, demo_rational_matrix_rref_fraction_free_low_rank);
    register_demo!(runner, demo_rational_matrix_rank);
    register_demo!(runner, demo_rational_matrix_rank_low_rank);
    register_demo!(runner, demo_rational_matrix_nullspace);
    register_demo!(runner, demo_rational_matrix_nullspace_low_rank);
    register_demo!(runner, demo_rational_matrix_reduced_row_echelon_form_assign);
    register_bench!(
        runner,
        benchmark_rational_matrix_reduced_row_echelon_form_algorithms
    );
}

fn rref_classical_of(a: &RationalMatrix) -> RationalMatrix {
    let mut a = a.clone();
    rref_classical(&mut a);
    a
}

fn rref_fraction_free_of(a: &RationalMatrix) -> RationalMatrix {
    let mut a = a.clone();
    rref_fraction_free(&mut a);
    a
}

fn demo_rational_matrix_reduced_row_echelon_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        println!(
            "reduced_row_echelon_form({a}) = {}",
            (&a).reduced_row_echelon_form()
        );
    }
}

// Matrices of rank at most the width of a second matrix, when that is smaller than their height and
// width.
fn demo_rational_matrix_reduced_row_echelon_form_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "reduced_row_echelon_form({a}) = {}",
            (&a).reduced_row_echelon_form()
        );
    }
}

fn demo_rational_matrix_rref_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        println!("rref_classical({a}) = {}", rref_classical_of(&a));
    }
}

// Matrices of rank at most the width of a second matrix, when that is smaller than their height and
// width.
fn demo_rational_matrix_rref_classical_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("rref_classical({a}) = {}", rref_classical_of(&a));
    }
}

fn demo_rational_matrix_rref_fraction_free(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        println!("rref_fraction_free({a}) = {}", rref_fraction_free_of(&a));
    }
}

// Matrices of rank at most the width of a second matrix, when that is smaller than their height and
// width.
fn demo_rational_matrix_rref_fraction_free_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("rref_fraction_free({a}) = {}", rref_fraction_free_of(&a));
    }
}

fn demo_rational_matrix_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        println!("rank({a}) = {}", (&a).rank());
    }
}

// Matrices of rank at most the width of a second matrix, when that is smaller than their height and
// width.
fn demo_rational_matrix_rank_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("rank({a}) = {}", (&a).rank());
    }
}

fn demo_rational_matrix_nullspace(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        println!("nullspace({a}) = {}", (&a).nullspace());
    }
}

// Matrices of rank at most the width of a second matrix, when that is smaller than their height and
// width.
fn demo_rational_matrix_nullspace_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("nullspace({a}) = {}", (&a).nullspace());
    }
}

fn demo_rational_matrix_reduced_row_echelon_form_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.reduced_row_echelon_form_assign();
        println!("a := {a_old}; a.reduced_row_echelon_form_assign(); a = {a}");
    }
}

fn benchmark_rational_matrix_reduced_row_echelon_form_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.reduced_row_echelon_form()",
        BenchmarkType::Algorithms,
        rational_matrix_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("classical", &mut |a| {
                no_out!(rref_classical_of(&a));
            }),
            ("fraction-free", &mut |a| {
                no_out!(rref_fraction_free_of(&a));
            }),
        ],
    );
}
