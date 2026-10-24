// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Rank;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::rank::rank_small;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_rank);
    register_demo!(runner, demo_integer_matrix_rank_val);
    register_demo!(runner, demo_integer_matrix_rank_low_rank);
    register_demo!(runner, demo_integer_matrix_rank_small);
    register_demo!(runner, demo_integer_matrix_rank_small_low_rank);
    register_bench!(runner, benchmark_integer_matrix_rank_evaluation_strategy);
}

// Whether every element of `a` has at most 30 bits, as `rank_small` requires.
fn small(a: &IntegerMatrix) -> bool {
    a.elements.iter().all(|x| x.significant_bits() <= 30)
}

fn rank_small_of(a: &IntegerMatrix) -> usize {
    rank_small(
        a.elements.iter().map(i64::exact_from).collect(),
        usize::exact_from(a.height()),
        usize::exact_from(a.width()),
    )
}

fn demo_integer_matrix_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("rank({a}) = {}", (&a).rank());
    }
}

fn demo_integer_matrix_rank_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.rank() = {}", a.rank());
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_rank_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("rank({a}) = {}", (&a).rank());
    }
}

// The matrices whose elements have at most 30 bits.
fn demo_integer_matrix_rank_small(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen()
        .get(gm, config)
        .filter(small)
        .take(limit)
    {
        println!("rank_small({a}) = {}", rank_small_of(&a));
    }
}

// Products, with elements of at most 30 bits, of matrices whose inner dimension is often smaller
// than the outer ones, so that many have less than full rank.
fn demo_integer_matrix_rank_small_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_pair_gen_var_4()
        .get(gm, config)
        .map(|(a, b)| a * b)
        .filter(small)
        .take(limit)
    {
        println!("rank_small({a}) = {}", rank_small_of(&a));
    }
}

fn benchmark_integer_matrix_rank_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.rank()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.rank()", &mut |a| {
                no_out!(a.rank());
            }),
            ("(&IntegerMatrix).rank()", &mut |a| {
                no_out!((&a).rank());
            }),
        ],
    );
}
