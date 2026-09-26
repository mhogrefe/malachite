// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModNeg, ModNegAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_neg);
    register_demo!(runner, demo_unsigned_polynomial_mod_neg_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_neg_assign);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_neg_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_neg({m}) = {}", p.mod_neg(m));
    }
}

fn demo_unsigned_polynomial_mod_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_neg({m}) = {}", (&p).mod_neg(m));
    }
}

fn demo_unsigned_polynomial_mod_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_neg_assign(m);
        println!("p := {p_old}; p.mod_neg_assign({m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_neg(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("UnsignedPolynomial<u64>.mod_neg(u64)", &mut |(p, _, m)| {
                no_out!(p.mod_neg(m));
            }),
            (
                "(&UnsignedPolynomial<u64>).mod_neg(u64)",
                &mut |(p, _, m)| {
                    no_out!((&p).mod_neg(m));
                },
            ),
            (
                "UnsignedPolynomial<u64>.mod_neg_assign(u64)",
                &mut |(mut p, _, m)| {
                    p.mod_neg_assign(m);
                },
            ),
        ],
    );
}
