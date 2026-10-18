// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_sub);
    register_demo!(runner, demo_integer_matrix_sub_val_ref);
    register_demo!(runner, demo_integer_matrix_sub_ref_val);
    register_demo!(runner, demo_integer_matrix_sub_ref_ref);
    register_demo!(runner, demo_integer_matrix_sub_assign);
    register_demo!(runner, demo_integer_matrix_sub_assign_ref);

    register_bench!(runner, benchmark_integer_matrix_sub_evaluation_strategy);
}

fn demo_integer_matrix_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!("{a_old} - {b_old} = {}", a - b);
    }
}

fn demo_integer_matrix_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old} - &{b} = {}", a - &b);
    }
}

fn demo_integer_matrix_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let b_old = b.clone();
        println!("&{a} - {b_old} = {}", &a - b);
    }
}

fn demo_integer_matrix_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        println!("&{a} - &{b} = {}", &a - &b);
    }
}

fn demo_integer_matrix_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in integer_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let (a_old, b_old) = (a.clone(), b.clone());
        a -= b;
        println!("a := {a_old}; a -= {b_old}; a = {a}");
    }
}

fn demo_integer_matrix_sub_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in integer_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        a -= &b;
        println!("a := {a_old}; a -= &{b}; a = {a}");
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_matrix_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix - IntegerMatrix",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("IntegerMatrix - IntegerMatrix", &mut |(a, b)| {
                no_out!(a - b);
            }),
            ("IntegerMatrix - &IntegerMatrix", &mut |(a, b)| {
                no_out!(a - &b);
            }),
            ("&IntegerMatrix - IntegerMatrix", &mut |(a, b)| {
                no_out!(&a - b);
            }),
            ("&IntegerMatrix - &IntegerMatrix", &mut |(a, b)| {
                no_out!(&a - &b);
            }),
            ("IntegerMatrix -= IntegerMatrix", &mut |(mut a, b)| a -= b),
            ("IntegerMatrix -= &IntegerMatrix", &mut |(mut a, b)| a -= &b),
        ],
    );
}
