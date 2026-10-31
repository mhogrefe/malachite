// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{CheckedDivTruncated, DivTruncated, DivTruncatedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_2_rational_polynomial_max_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1,
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4,
};
use malachite_q::test_util::rational_polynomial::arithmetic::div_truncated::div_truncated_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_div_truncated);
    register_demo!(runner, demo_rational_polynomial_div_truncated_ref_ref);
    register_demo!(runner, demo_rational_polynomial_div_truncated_assign);
    register_demo!(runner, demo_rational_polynomial_checked_div_truncated);
    register_bench!(
        runner,
        benchmark_rational_polynomial_div_truncated_algorithms
    );
}

fn demo_rational_polynomial_div_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).div_truncated({q_old}, {len}) = {}",
            p.div_truncated(q, len)
        );
    }
}

fn demo_rational_polynomial_div_truncated_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).div_truncated(&({q}), {len}) = {}",
            (&p).div_truncated(&q, len)
        );
    }
}

fn demo_rational_polynomial_div_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.div_truncated_assign(q, len);
        println!("p := {p_old}; p.div_truncated_assign({q_old}, {len}); p = {p}");
    }
}

fn demo_rational_polynomial_checked_div_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_div_truncated(&({q}), {len}) = {:?}",
            (&p).checked_div_truncated(&q, len)
        );
    }
}

fn benchmark_rational_polynomial_div_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.div_truncated(RationalPolynomial, u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_rational_polynomial_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, len)| {
                no_out!((&p).div_truncated(&q, len));
            }),
            ("naive", &mut |(p, q, len)| {
                no_out!(div_truncated_naive(&p, &q, len));
            }),
        ],
    );
}
