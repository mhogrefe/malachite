// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModAssign};
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_op);
    register_demo!(runner, demo_unsigned_polynomial_mod_assign);
    register_demo!(runner, demo_unsigned_polynomial_rem);
    register_demo!(runner, demo_unsigned_polynomial_rem_ref);
    register_demo!(runner, demo_unsigned_polynomial_rem_assign);

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_rem_evaluation_strategy
    );
}

// The generator's second value can be zero, which is not a divisor, so the demos and benchmarks
// step past it.
fn demo_unsigned_polynomial_mod_op(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m = m + 1;
        let p_old = p.clone();
        println!("({p_old}).mod_op({m}) = {}", p.mod_op(m));
    }
}

fn demo_unsigned_polynomial_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, m) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m = m + 1;
        let p_old = p.clone();
        p.mod_assign(m);
        println!("p := {p_old}; p.mod_assign({m}); p = {p}");
    }
}

fn demo_unsigned_polynomial_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m = m + 1;
        let p_old = p.clone();
        println!("({p_old}) % {m} = {}", p % m);
    }
}

fn demo_unsigned_polynomial_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m = m + 1;
        println!("&({p}) % {m} = {}", &p % m);
    }
}

fn demo_unsigned_polynomial_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, m) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m = m + 1;
        let p_old = p.clone();
        p %= m;
        println!("p := {p_old}; p %= {m}; p = {p}");
    }
}

// The remainders are what is being timed, so the benchmark discards them on purpose.
#[allow(unused_must_use)]
fn benchmark_unsigned_polynomial_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64> % u64",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_polynomial_bit_bucketer("p"),
        &mut [
            ("UnsignedPolynomial<u64> % u64", &mut |(p, m)| {
                no_out!(p % (m + 1));
            }),
            ("&UnsignedPolynomial<u64> % u64", &mut |(p, m)| {
                no_out!(&p % (m + 1));
            }),
            ("UnsignedPolynomial<u64> %= u64", &mut |(mut p, m)| {
                p %= m + 1;
            }),
            ("UnsignedPolynomial<u64>.mod_op(u64)", &mut |(p, m)| {
                no_out!(p.mod_op(m + 1));
            }),
        ],
    );
}
