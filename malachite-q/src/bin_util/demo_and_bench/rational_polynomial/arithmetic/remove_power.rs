// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_pair_gen_var_5;
use malachite_q::test_util::rational_polynomial::arithmetic::div_rem::remove_power_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_remove_power);
    register_demo!(runner, demo_rational_polynomial_remove_power_ref_ref);
    register_demo!(runner, demo_rational_polynomial_remove_power_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_remove_power_algorithms
    );
}

fn demo_rational_polynomial_remove_power(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).remove_power({q_old}) = {:?}", p.remove_power(q));
    }
}

fn demo_rational_polynomial_remove_power_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in rational_polynomial_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).remove_power(&({q})) = {:?}",
            (&p).remove_power(&q)
        );
    }
}

fn demo_rational_polynomial_remove_power_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in rational_polynomial_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let e = p.remove_power_assign(q);
        println!("p := {p_old}; p.remove_power_assign({q_old}) = {e}; p = {p}");
    }
}

fn benchmark_rational_polynomial_remove_power_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.remove_power(RationalPolynomial)",
        BenchmarkType::Algorithms,
        rational_polynomial_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).remove_power(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(remove_power_naive(&p, &q));
            }),
        ],
    );
}
