// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EntrywiseMax, EntrywiseMaxAssign};
use malachite_base::test_util::bench::bucketers::pair_unsigned_vector_max_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_entrywise_max);
    register_demo!(runner, demo_unsigned_vector_entrywise_max_ref_ref);
    register_demo!(runner, demo_unsigned_vector_entrywise_max_assign);

    register_bench!(
        runner,
        benchmark_unsigned_vector_entrywise_max_evaluation_strategy
    );
}

fn demo_unsigned_vector_entrywise_max(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in unsigned_vector_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.entrywise_max({w_old}) = {}", v.entrywise_max(w));
    }
}

fn demo_unsigned_vector_entrywise_max_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in unsigned_vector_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).entrywise_max(&{w}) = {}", (&v).entrywise_max(&w));
    }
}

fn demo_unsigned_vector_entrywise_max_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w) in unsigned_vector_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.entrywise_max_assign(&w);
        println!("v := {v_old}; v.entrywise_max_assign(&{w}); v = {v}");
    }
}

fn benchmark_unsigned_vector_entrywise_max_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.entrywise_max(UnsignedVector<u64>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_unsigned_vector_max_dimension_bucketer("v", "w"),
        &mut [
            (
                "UnsignedVector<u64>.entrywise_max(UnsignedVector<u64>)",
                &mut |(v, w)| {
                    no_out!(v.entrywise_max(w));
                },
            ),
            (
                "UnsignedVector<u64>.entrywise_max(&UnsignedVector<u64>)",
                &mut |(v, w)| {
                    no_out!(v.entrywise_max(&w));
                },
            ),
            (
                "(&UnsignedVector<u64>).entrywise_max(UnsignedVector<u64>)",
                &mut |(v, w)| {
                    no_out!((&v).entrywise_max(w));
                },
            ),
            (
                "(&UnsignedVector<u64>).entrywise_max(&UnsignedVector<u64>)",
                &mut |(v, w)| {
                    no_out!((&v).entrywise_max(&w));
                },
            ),
            (
                "UnsignedVector<u64>.entrywise_max_assign(UnsignedVector<u64>)",
                &mut |(mut v, w)| {
                    v.entrywise_max_assign(w);
                },
            ),
            (
                "UnsignedVector<u64>.entrywise_max_assign(&UnsignedVector<u64>)",
                &mut |(mut v, w)| {
                    v.entrywise_max_assign(&w);
                },
            ),
        ],
    );
}
