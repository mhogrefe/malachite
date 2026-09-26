// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModMakeMonic, ModMakeMonicAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_make_monic);
    register_demo!(runner, demo_unsigned_polynomial_mod_make_monic_assign);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_make_monic_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_make_monic(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).mod_make_monic({m}) = {:?}",
            (&p).mod_make_monic(m).map(|q| q.to_string())
        );
    }
}

fn demo_unsigned_polynomial_mod_make_monic_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, _, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let result = p.mod_make_monic_assign(m);
        println!("p := {p_old}; p.mod_make_monic_assign({m}) = {result:?}; p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_make_monic_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_make_monic(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_make_monic(u64)",
                &mut |(p, _, m)| {
                    let _ = p.mod_make_monic(m);
                },
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_make_monic(u64)",
                &mut |(p, _, m)| {
                    let _ = (&p).mod_make_monic(m);
                },
            ),
            (
                "UnsignedPolynomial<u64>.mod_make_monic_assign(u64)",
                &mut |(mut p, _, m)| {
                    let _ = p.mod_make_monic_assign(m);
                },
            ),
        ],
    );
}
