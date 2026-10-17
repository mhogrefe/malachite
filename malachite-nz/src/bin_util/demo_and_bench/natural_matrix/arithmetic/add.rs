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
use malachite_nz::test_util::bench::bucketers::pair_natural_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_add);
    register_demo!(runner, demo_natural_matrix_add_val_ref);
    register_demo!(runner, demo_natural_matrix_add_ref_val);
    register_demo!(runner, demo_natural_matrix_add_ref_ref);
    register_demo!(runner, demo_natural_matrix_add_assign);
    register_demo!(runner, demo_natural_matrix_add_assign_ref);

    register_bench!(runner, benchmark_natural_matrix_add_evaluation_strategy);
}

fn demo_natural_matrix_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let (v_old, w_old) = (a.clone(), b.clone());
        println!("{v_old} + {w_old} = {}", a + b);
    }
}

fn demo_natural_matrix_add_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = a.clone();
        println!("{v_old} + &{b} = {}", a + &b);
    }
}

fn demo_natural_matrix_add_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let w_old = b.clone();
        println!("&{a} + {w_old} = {}", &a + b);
    }
}

fn demo_natural_matrix_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in natural_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        println!("&{a} + &{b} = {}", &a + &b);
    }
}

fn demo_natural_matrix_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in natural_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let (v_old, w_old) = (a.clone(), b.clone());
        a += b;
        println!("a := {v_old}; a += {w_old}; a = {a}");
    }
}

fn demo_natural_matrix_add_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in natural_matrix_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = a.clone();
        a += &b;
        println!("a := {v_old}; a += &{b}; a = {a}");
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_matrix_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix + NaturalMatrix",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("NaturalMatrix + NaturalMatrix", &mut |(a, b)| {
                no_out!(a + b);
            }),
            ("NaturalMatrix + &NaturalMatrix", &mut |(a, b)| {
                no_out!(a + &b);
            }),
            ("&NaturalMatrix + NaturalMatrix", &mut |(a, b)| {
                no_out!(&a + b);
            }),
            ("&NaturalMatrix + &NaturalMatrix", &mut |(a, b)| {
                no_out!(&a + &b);
            }),
            ("NaturalMatrix += NaturalMatrix", &mut |(mut a, b)| a += b),
            ("NaturalMatrix += &NaturalMatrix", &mut |(mut a, b)| a += &b),
        ],
    );
}
