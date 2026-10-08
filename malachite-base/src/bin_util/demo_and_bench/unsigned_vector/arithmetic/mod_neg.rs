// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModNeg, ModNegAssign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_neg);
    register_demo!(runner, demo_unsigned_vector_mod_neg_ref);
    register_demo!(runner, demo_unsigned_vector_mod_neg_assign);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_neg_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in unsigned_vector_unsigned_pair_gen_var_5::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("({v_old}).mod_neg({m}) = {}", v.mod_neg(m));
    }
}

fn demo_unsigned_vector_mod_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in unsigned_vector_unsigned_pair_gen_var_5::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_neg({m}) = {}", (&v).mod_neg(m));
    }
}

fn demo_unsigned_vector_mod_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, m) in unsigned_vector_unsigned_pair_gen_var_5::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_neg_assign(m);
        println!("v := {v_old}; v.mod_neg_assign({m}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_neg(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_pair_gen_var_5::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.mod_neg(u64)", &mut |(v, m)| {
                no_out!(v.mod_neg(m));
            }),
            ("(&UnsignedVector<u64>).mod_neg(u64)", &mut |(v, m)| {
                no_out!((&v).mod_neg(m));
            }),
            (
                "UnsignedVector<u64>.mod_neg_assign(u64)",
                &mut |(mut v, m)| {
                    v.mod_neg_assign(m);
                },
            ),
        ],
    );
}
