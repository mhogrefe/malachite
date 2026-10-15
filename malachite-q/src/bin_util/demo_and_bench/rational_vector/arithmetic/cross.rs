// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Cross, CrossAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_vector_max_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_pair_gen_var_2;
use malachite_q::test_util::rational_vector::arithmetic::cross::rational_vector_cross_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_cross);
    register_demo!(runner, demo_rational_vector_cross_val_ref);
    register_demo!(runner, demo_rational_vector_cross_ref_val);
    register_demo!(runner, demo_rational_vector_cross_ref_ref);
    register_demo!(runner, demo_rational_vector_cross_assign);
    register_demo!(runner, demo_rational_vector_cross_assign_ref);
    register_bench!(runner, benchmark_rational_vector_cross_evaluation_strategy);
    register_bench!(runner, benchmark_rational_vector_cross_algorithms);
}

fn demo_rational_vector_cross(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_2().get(gm, config).take(limit) {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.cross({w_old}) = {}", v.cross(w));
    }
}

fn demo_rational_vector_cross_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_2().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.cross(&{w}) = {}", v.cross(&w));
    }
}

fn demo_rational_vector_cross_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_2().get(gm, config).take(limit) {
        let w_old = w.clone();
        println!("(&{v}).cross({w_old}) = {}", (&v).cross(w));
    }
}

fn demo_rational_vector_cross_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_2().get(gm, config).take(limit) {
        println!("(&{v}).cross(&{w}) = {}", (&v).cross(&w));
    }
}

fn demo_rational_vector_cross_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in rational_vector_pair_gen_var_2().get(gm, config).take(limit) {
        let v_old = v.clone();
        let w_old = w.clone();
        v.cross_assign(w);
        println!("v := {v_old}; v.cross_assign({w_old}); v = {v}");
    }
}

fn demo_rational_vector_cross_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in rational_vector_pair_gen_var_2().get(gm, config).take(limit) {
        let v_old = v.clone();
        v.cross_assign(&w);
        println!("v := {v_old}; v.cross_assign(&{w}); v = {v}");
    }
}

fn benchmark_rational_vector_cross_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.cross(RationalVector)",
        BenchmarkType::EvaluationStrategy,
        rational_vector_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("RationalVector.cross(RationalVector)", &mut |(v, w)| {
                no_out!(v.cross(w));
            }),
            ("RationalVector.cross(&RationalVector)", &mut |(v, w)| {
                no_out!(v.cross(&w));
            }),
            ("(&RationalVector).cross(RationalVector)", &mut |(v, w)| {
                no_out!((&v).cross(w));
            }),
            ("(&RationalVector).cross(&RationalVector)", &mut |(v, w)| {
                no_out!((&v).cross(&w));
            }),
            (
                "RationalVector.cross_assign(RationalVector)",
                &mut |(mut v, w)| {
                    v.cross_assign(w);
                },
            ),
            (
                "RationalVector.cross_assign(&RationalVector)",
                &mut |(mut v, w)| {
                    v.cross_assign(&w);
                },
            ),
        ],
    );
}

fn benchmark_rational_vector_cross_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.cross(RationalVector)",
        BenchmarkType::Algorithms,
        rational_vector_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("default", &mut |(v, w)| {
                no_out!((&v).cross(&w));
            }),
            ("naive", &mut |(v, w)| {
                no_out!(rational_vector_cross_naive(&v, &w));
            }),
        ],
    );
}
