// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_pow);
    register_demo!(runner, demo_unsigned_polynomial_mod_pow_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_pow_assign);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_pow_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_polynomial_mod_pow_algorithms);
}

fn demo_unsigned_polynomial_mod_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_pow({e}, {m}) = {}", p.mod_pow(e, m));
    }
}

fn demo_unsigned_polynomial_mod_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_pow({e}, {m}) = {}", (&p).mod_pow(e, m));
    }
}

fn demo_unsigned_polynomial_mod_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, e, m) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_pow_assign(e, m);
        println!("p := {p_old}; p.mod_pow_assign({e}, {m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_pow(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_pow(u64, u64)",
                &mut |(p, e, m)| no_out!(p.mod_pow(e, m)),
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_pow(u64, u64)",
                &mut |(p, e, m)| no_out!((&p).mod_pow(e, m)),
            ),
            (
                "UnsignedPolynomial<u64>.mod_pow_assign(u64, u64)",
                &mut |(mut p, e, m)| p.mod_pow_assign(e, m),
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).mod_pow(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_8::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e, m)| {
                no_out!((&p).mod_pow(e, m));
            }),
            ("naive", &mut |(p, e, m)| {
                no_out!(mod_pow_naive(&p, e, m));
            }),
        ],
    );
}
