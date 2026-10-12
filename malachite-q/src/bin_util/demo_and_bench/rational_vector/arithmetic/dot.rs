// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Dot;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_vector_max_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_pair_gen_var_1;
use malachite_q::test_util::rational_vector::arithmetic::dot::rational_vector_dot_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_dot);
    register_demo!(runner, demo_rational_vector_dot_val_ref);
    register_demo!(runner, demo_rational_vector_dot_ref_val);
    register_demo!(runner, demo_rational_vector_dot_ref_ref);
    register_bench!(runner, benchmark_rational_vector_dot_algorithms);
    register_bench!(runner, benchmark_rational_vector_dot_evaluation_strategy);
}

fn demo_rational_vector_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.dot({w_old}) = {}", v.dot(w));
    }
}

fn demo_rational_vector_dot_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.dot(&{w}) = {}", v.dot(&w));
    }
}

fn demo_rational_vector_dot_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let w_old = w.clone();
        println!("(&{v}).dot({w_old}) = {}", (&v).dot(w));
    }
}

fn demo_rational_vector_dot_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{v}).dot(&{w}) = {}", (&v).dot(&w));
    }
}

fn benchmark_rational_vector_dot_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.dot(RationalVector)",
        BenchmarkType::Algorithms,
        rational_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("default", &mut |(v, w)| {
                no_out!(v.dot(w));
            }),
            ("naive", &mut |(v, w)| {
                no_out!(rational_vector_dot_naive(&v, &w));
            }),
        ],
    );
}

fn benchmark_rational_vector_dot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.dot(RationalVector)",
        BenchmarkType::EvaluationStrategy,
        rational_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("RationalVector.dot(RationalVector)", &mut |(v, w)| {
                no_out!(v.dot(w));
            }),
            ("RationalVector.dot(&RationalVector)", &mut |(v, w)| {
                no_out!(v.dot(&w));
            }),
            ("(&RationalVector).dot(RationalVector)", &mut |(v, w)| {
                no_out!((&v).dot(w));
            }),
            ("(&RationalVector).dot(&RationalVector)", &mut |(v, w)| {
                no_out!((&v).dot(&w));
            }),
        ],
    );
}
