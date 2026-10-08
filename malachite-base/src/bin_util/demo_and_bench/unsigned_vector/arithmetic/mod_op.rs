// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModAssign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_3;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_op);
    register_demo!(runner, demo_unsigned_vector_mod_assign);
    register_demo!(runner, demo_unsigned_vector_rem);
    register_demo!(runner, demo_unsigned_vector_rem_ref);
    register_demo!(runner, demo_unsigned_vector_rem_assign);

    register_bench!(runner, benchmark_unsigned_vector_rem_evaluation_strategy);
}

fn demo_unsigned_vector_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in unsigned_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("({v_old}).mod_op({m}) = {}", v.mod_op(m));
    }
}

fn demo_unsigned_vector_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, m) in unsigned_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_assign(m);
        println!("v := {v_old}; v.mod_assign({m}); v = {v}");
    }
}

fn demo_unsigned_vector_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in unsigned_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} % {m} = {}", v % m);
    }
}

fn demo_unsigned_vector_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in unsigned_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} % {m} = {}", &v % m);
    }
}

fn demo_unsigned_vector_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, m) in unsigned_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v %= m;
        println!("v := {v_old}; v %= {m}; v = {v}");
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_unsigned_vector_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64> % u64",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64> % u64", &mut |(v, m)| no_out!(v % m)),
            ("&UnsignedVector<u64> % u64", &mut |(v, m)| no_out!(&v % m)),
            ("UnsignedVector<u64> %= u64", &mut |(mut v, m)| v %= m),
            ("UnsignedVector<u64>.mod_op(u64)", &mut |(v, m)| {
                no_out!(v.mod_op(m));
            }),
        ],
    );
}
