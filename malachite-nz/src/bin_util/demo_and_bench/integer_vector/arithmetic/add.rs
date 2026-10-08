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
use malachite_nz::test_util::bench::bucketers::pair_integer_vector_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_add);
    register_demo!(runner, demo_integer_vector_add_val_ref);
    register_demo!(runner, demo_integer_vector_add_ref_val);
    register_demo!(runner, demo_integer_vector_add_ref_ref);
    register_demo!(runner, demo_integer_vector_add_assign);
    register_demo!(runner, demo_integer_vector_add_assign_ref);

    register_bench!(runner, benchmark_integer_vector_add_evaluation_strategy);
}

fn demo_integer_vector_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let (v_old, w_old) = (v.clone(), w.clone());
        println!("{v_old} + {w_old} = {}", v + w);
    }
}

fn demo_integer_vector_add_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old} + &{w} = {}", v + &w);
    }
}

fn demo_integer_vector_add_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let w_old = w.clone();
        println!("&{v} + {w_old} = {}", &v + w);
    }
}

fn demo_integer_vector_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        println!("&{v} + &{w} = {}", &v + &w);
    }
}

fn demo_integer_vector_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let (v_old, w_old) = (v.clone(), w.clone());
        v += w;
        println!("v := {v_old}; v += {w_old}; v = {v}");
    }
}

fn demo_integer_vector_add_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        v += &w;
        println!("v := {v_old}; v += &{w}; v = {v}");
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_vector_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector + IntegerVector",
        BenchmarkType::EvaluationStrategy,
        integer_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("IntegerVector + IntegerVector", &mut |(v, w)| {
                no_out!(v + w);
            }),
            ("IntegerVector + &IntegerVector", &mut |(v, w)| {
                no_out!(v + &w);
            }),
            ("&IntegerVector + IntegerVector", &mut |(v, w)| {
                no_out!(&v + w);
            }),
            ("&IntegerVector + &IntegerVector", &mut |(v, w)| {
                no_out!(&v + &w);
            }),
            ("IntegerVector += IntegerVector", &mut |(mut v, w)| v += w),
            ("IntegerVector += &IntegerVector", &mut |(mut v, w)| v += &w),
        ],
    );
}
