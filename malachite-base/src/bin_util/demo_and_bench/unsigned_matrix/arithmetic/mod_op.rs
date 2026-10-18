// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModAssign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_op);
    register_demo!(runner, demo_unsigned_matrix_mod_assign);
    register_demo!(runner, demo_unsigned_matrix_rem);
    register_demo!(runner, demo_unsigned_matrix_rem_ref);
    register_demo!(runner, demo_unsigned_matrix_rem_assign);

    register_bench!(runner, benchmark_unsigned_matrix_rem_evaluation_strategy);
}

fn demo_unsigned_matrix_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("({a_old}).mod_op({m}) = {}", a.mod_op(m));
    }
}

fn demo_unsigned_matrix_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in unsigned_matrix_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_assign(m);
        println!("a := {a_old}; a.mod_assign({m}); a = {a}");
    }
}

fn demo_unsigned_matrix_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} % {m} = {}", a % m);
    }
}

fn demo_unsigned_matrix_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} % {m} = {}", &a % m);
    }
}

fn demo_unsigned_matrix_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, m) in unsigned_matrix_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a %= m;
        println!("a := {a_old}; a %= {m}; a = {a}");
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_unsigned_matrix_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64> % u64",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("UnsignedMatrix<u64> % u64", &mut |(a, m)| no_out!(a % m)),
            ("&UnsignedMatrix<u64> % u64", &mut |(a, m)| no_out!(&a % m)),
            ("UnsignedMatrix<u64> %= u64", &mut |(mut a, m)| a %= m),
            ("UnsignedMatrix<u64>.mod_op(u64)", &mut |(a, m)| {
                no_out!(a.mod_op(m));
            }),
        ],
    );
}
