// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModDot;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_dot);
    register_demo!(runner, demo_unsigned_vector_mod_dot_ref_ref);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_dot_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, m) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.mod_dot({w_old}, {m}) = {}", v.mod_dot(w, m));
    }
}

fn demo_unsigned_vector_mod_dot_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, m) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_dot(&{w}, {m}) = {}", (&v).mod_dot(&w, m));
    }
}

fn benchmark_unsigned_vector_mod_dot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_dot(UnsignedVector<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector<u64>.mod_dot(UnsignedVector<u64>, u64)",
                &mut |(v, w, m)| {
                    no_out!(v.mod_dot(w, m));
                },
            ),
            (
                "UnsignedVector<u64>.mod_dot(&UnsignedVector<u64>, u64)",
                &mut |(v, w, m)| {
                    no_out!(v.mod_dot(&w, m));
                },
            ),
            (
                "(&UnsignedVector<u64>).mod_dot(UnsignedVector<u64>, u64)",
                &mut |(v, w, m)| {
                    no_out!((&v).mod_dot(w, m));
                },
            ),
            (
                "(&UnsignedVector<u64>).mod_dot(&UnsignedVector<u64>, u64)",
                &mut |(v, w, m)| {
                    no_out!((&v).mod_dot(&w, m));
                },
            ),
        ],
    );
}
