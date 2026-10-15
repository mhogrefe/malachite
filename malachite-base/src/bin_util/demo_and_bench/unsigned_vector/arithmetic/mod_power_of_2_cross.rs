// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Cross, ModPowerOf2CrossAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_power_of_2_cross::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_cross);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_cross_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_cross_assign);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_cross_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_cross_algorithms
    );
}

fn demo_unsigned_vector_mod_power_of_2_cross<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, pow) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!(
            "{v_old}.mod_power_of_2_cross({w_old}, {pow}) = {}",
            v.mod_power_of_2_cross(w, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_cross_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, pow) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_cross(&{w}, {pow}) = {}",
            (&v).mod_power_of_2_cross(&w, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_cross_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut v, w, pow) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_cross_assign(&w, pow);
        println!("v := {v_old}; v.mod_power_of_2_cross_assign(&{w}, {pow}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_cross_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_power_of_2_cross(UnsignedVector<{}>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("v.mod_power_of_2_cross(w, pow)", &mut |(v, w, pow)| {
                no_out!(v.mod_power_of_2_cross(w, pow));
            }),
            ("v.mod_power_of_2_cross(&w, pow)", &mut |(v, w, pow)| {
                no_out!(v.mod_power_of_2_cross(&w, pow));
            }),
            ("(&v).mod_power_of_2_cross(w, pow)", &mut |(v, w, pow)| {
                no_out!((&v).mod_power_of_2_cross(w, pow));
            }),
            ("(&v).mod_power_of_2_cross(&w, pow)", &mut |(v, w, pow)| {
                no_out!((&v).mod_power_of_2_cross(&w, pow));
            }),
            (
                "v.mod_power_of_2_cross_assign(w, pow)",
                &mut |(mut v, w, pow)| {
                    v.mod_power_of_2_cross_assign(w, pow);
                },
            ),
            (
                "v.mod_power_of_2_cross_assign(&w, pow)",
                &mut |(mut v, w, pow)| {
                    v.mod_power_of_2_cross_assign(&w, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_vector_mod_power_of_2_cross_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_power_of_2_cross(UnsignedVector<{}>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(v, w, pow)| {
                no_out!((&v).mod_power_of_2_cross(&w, pow));
            }),
            ("naive", &mut |(v, w, pow)| {
                no_out!(unsigned_vector_mod_power_of_2_cross_naive(&v, &w, pow));
            }),
        ],
    );
}
