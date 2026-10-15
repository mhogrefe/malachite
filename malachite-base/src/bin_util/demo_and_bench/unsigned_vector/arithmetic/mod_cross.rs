// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModCross, ModCrossAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_cross::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_cross);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_cross_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_cross_assign);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_cross_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_unsigned_vector_mod_cross_algorithms);
}

fn demo_unsigned_vector_mod_cross<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, m) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.mod_cross({w_old}, {m}) = {}", v.mod_cross(w, m));
    }
}

fn demo_unsigned_vector_mod_cross_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, m) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_cross(&{w}, {m}) = {}", (&v).mod_cross(&w, m));
    }
}

fn demo_unsigned_vector_mod_cross_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut v, w, m) in unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_cross_assign(&w, m);
        println!("v := {v_old}; v.mod_cross_assign(&{w}, {m}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_cross_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_cross(UnsignedVector<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("v.mod_cross(w, m)", &mut |(v, w, m)| {
                no_out!(v.mod_cross(w, m));
            }),
            ("v.mod_cross(&w, m)", &mut |(v, w, m)| {
                no_out!(v.mod_cross(&w, m));
            }),
            ("(&v).mod_cross(w, m)", &mut |(v, w, m)| {
                no_out!((&v).mod_cross(w, m));
            }),
            ("(&v).mod_cross(&w, m)", &mut |(v, w, m)| {
                no_out!((&v).mod_cross(&w, m));
            }),
            ("v.mod_cross_assign(w, m)", &mut |(mut v, w, m)| {
                v.mod_cross_assign(w, m);
            }),
            ("v.mod_cross_assign(&w, m)", &mut |(mut v, w, m)| {
                v.mod_cross_assign(&w, m);
            }),
        ],
    );
}

fn benchmark_unsigned_vector_mod_cross_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_cross(UnsignedVector<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(v, w, m)| {
                no_out!((&v).mod_cross(&w, m));
            }),
            ("naive", &mut |(v, w, m)| {
                no_out!(unsigned_vector_mod_cross_naive(&v, &w, m));
            }),
        ],
    );
}
