// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Lcm, LcmAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_7;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_lcm);
    register_demo!(runner, demo_integer_polynomial_lcm_ref_ref);
    register_demo!(runner, demo_integer_polynomial_lcm_assign);
    register_bench!(runner, benchmark_integer_polynomial_lcm_evaluation_strategy);
}

fn demo_integer_polynomial_lcm(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).lcm({q_old}) = {}", p.lcm(q));
    }
}

fn demo_integer_polynomial_lcm_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).lcm(&({q})) = {}", (&p).lcm(&q));
    }
}

fn demo_integer_polynomial_lcm_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.lcm_assign(q);
        println!("p := {p_old}; p.lcm_assign({q_old}); p = {p}");
    }
}

fn benchmark_integer_polynomial_lcm_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.lcm(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.lcm(q)", &mut |(p, q)| {
                no_out!(p.lcm(q));
            }),
            ("p.lcm(&q)", &mut |(p, q)| {
                no_out!(p.lcm(&q));
            }),
            ("(&p).lcm(q)", &mut |(p, q)| {
                no_out!((&p).lcm(q));
            }),
            ("(&p).lcm(&q)", &mut |(p, q)| {
                no_out!((&p).lcm(&q));
            }),
            ("p.lcm_assign(q)", &mut |(mut p, q)| {
                no_out!(p.lcm_assign(q));
            }),
            ("p.lcm_assign(&q)", &mut |(mut p, q)| {
                no_out!(p.lcm_assign(&q));
            }),
        ],
    );
}
