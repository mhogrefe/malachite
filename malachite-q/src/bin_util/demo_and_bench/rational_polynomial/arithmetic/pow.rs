// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_1;
use malachite_q::test_util::rational_polynomial::arithmetic::pow::pow_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_pow);
    register_demo!(runner, demo_rational_polynomial_pow_ref);
    register_demo!(runner, demo_rational_polynomial_pow_assign);

    register_bench!(
        runner,
        benchmark_rational_polynomial_pow_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_polynomial_pow_algorithms);
}

fn demo_rational_polynomial_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).pow({e}) = {}", p.pow(e));
    }
}

fn demo_rational_polynomial_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).pow({e}) = {}", (&p).pow(e));
    }
}

fn demo_rational_polynomial_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, e) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.pow_assign(e);
        println!("p := {p_old}; p.pow_assign({e}); p = {p}");
    }
}

fn benchmark_rational_polynomial_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("exp"),
        &mut [
            ("RationalPolynomial.pow(u64)", &mut |(p, e)| {
                no_out!(p.pow(e));
            }),
            ("(&RationalPolynomial).pow(u64)", &mut |(p, e)| {
                no_out!((&p).pow(e));
            }),
            ("RationalPolynomial.pow_assign(u64)", &mut |(mut p, e)| {
                p.pow_assign(e);
            }),
        ],
    );
}

fn benchmark_rational_polynomial_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalPolynomial).pow(u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e)| {
                no_out!((&p).pow(e));
            }),
            ("naive", &mut |(p, e)| {
                no_out!(pow_naive(&p, e));
            }),
        ],
    );
}
