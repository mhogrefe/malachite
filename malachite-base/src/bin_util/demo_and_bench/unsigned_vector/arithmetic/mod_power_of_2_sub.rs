// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Sub, ModPowerOf2SubAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_sub);
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_sub_ref_ref);
    register_demo!(runner, demo_unsigned_vector_mod_power_of_2_sub_assign);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_sub_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_power_of_2_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, pow) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let (v_old, w_old) = (v.clone(), w.clone());
        println!(
            "{v_old}.mod_power_of_2_sub({w_old}, {pow}) = {}",
            v.mod_power_of_2_sub(w, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, pow) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_sub(&{w}, {pow}) = {}",
            (&v).mod_power_of_2_sub(&w, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, w, pow) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_sub_assign(&w, pow);
        println!("v := {v_old}; v.mod_power_of_2_sub_assign(&{w}, {pow}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_power_of_2_sub(UnsignedVector<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector<u64>.mod_power_of_2_sub(UnsignedVector<u64>, u64)",
                &mut |(v, w, pow)| {
                    no_out!(v.mod_power_of_2_sub(w, pow));
                },
            ),
            (
                "UnsignedVector<u64>.mod_power_of_2_sub(&UnsignedVector<u64>, u64)",
                &mut |(v, w, pow)| {
                    no_out!(v.mod_power_of_2_sub(&w, pow));
                },
            ),
            (
                "(&UnsignedVector<u64>).mod_power_of_2_sub(UnsignedVector<u64>, u64)",
                &mut |(v, w, pow)| {
                    no_out!((&v).mod_power_of_2_sub(w, pow));
                },
            ),
            (
                "(&UnsignedVector<u64>).mod_power_of_2_sub(&UnsignedVector<u64>, u64)",
                &mut |(v, w, pow)| {
                    no_out!((&v).mod_power_of_2_sub(&w, pow));
                },
            ),
            (
                "UnsignedVector<u64>.mod_power_of_2_sub_assign(UnsignedVector<u64>, u64)",
                &mut |(mut v, w, pow)| {
                    v.mod_power_of_2_sub_assign(w, pow);
                },
            ),
            (
                "UnsignedVector<u64>.mod_power_of_2_sub_assign(&UnsignedVector<u64>, u64)",
                &mut |(mut v, w, pow)| {
                    v.mod_power_of_2_sub_assign(&w, pow);
                },
            ),
        ],
    );
}
