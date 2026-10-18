// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_add);
    register_demo!(runner, demo_unsigned_matrix_mod_add_ref_ref);
    register_demo!(runner, demo_unsigned_matrix_mod_add_assign);
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_add_evaluation_strategy
    );
}

fn demo_unsigned_matrix_mod_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (a_old, b_old) = (a.clone(), b.clone());
        println!("{a_old}.mod_add({b_old}, {m}) = {}", a.mod_add(b, m));
    }
}

fn demo_unsigned_matrix_mod_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_add(&{b}, {m}) = {}", (&a).mod_add(&b, m));
    }
}

fn demo_unsigned_matrix_mod_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, m) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_add_assign(&b, m);
        println!("a := {a_old}; a.mod_add_assign(&{b}, {m}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix<u64>.mod_add(UnsignedMatrix<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix<u64>.mod_add(UnsignedMatrix<u64>, u64)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_add(b, m));
                },
            ),
            (
                "UnsignedMatrix<u64>.mod_add(&UnsignedMatrix<u64>, u64)",
                &mut |(a, b, m)| {
                    no_out!(a.mod_add(&b, m));
                },
            ),
            (
                "(&UnsignedMatrix<u64>).mod_add(UnsignedMatrix<u64>, u64)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_add(b, m));
                },
            ),
            (
                "(&UnsignedMatrix<u64>).mod_add(&UnsignedMatrix<u64>, u64)",
                &mut |(a, b, m)| {
                    no_out!((&a).mod_add(&b, m));
                },
            ),
            (
                "UnsignedMatrix<u64>.mod_add_assign(UnsignedMatrix<u64>, u64)",
                &mut |(mut a, b, m)| {
                    a.mod_add_assign(b, m);
                },
            ),
            (
                "UnsignedMatrix<u64>.mod_add_assign(&UnsignedMatrix<u64>, u64)",
                &mut |(mut a, b, m)| {
                    a.mod_add_assign(&b, m);
                },
            ),
        ],
    );
}
