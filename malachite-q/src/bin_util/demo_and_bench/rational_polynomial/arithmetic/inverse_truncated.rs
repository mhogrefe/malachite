// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::InverseTruncated;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_polynomial::arithmetic::inverse_truncated::*;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_unsigned_pair_gen_var_1, rational_polynomial_unsigned_pair_gen_var_4,
};
use malachite_q::test_util::rational_polynomial::arithmetic::div_truncated::inverse_truncated_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_inverse_truncated);
    register_demo!(runner, demo_rational_polynomial_inverse_truncated_ref);
    register_demo!(
        runner,
        demo_rational_polynomial_inverse_truncated_invertible
    );
    register_bench!(
        runner,
        benchmark_rational_polynomial_inverse_truncated_algorithms
    );
}

fn demo_rational_polynomial_inverse_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).inverse_truncated({len}) = {:?}",
            p.inverse_truncated(len)
        );
    }
}

fn demo_rational_polynomial_inverse_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).inverse_truncated({len}) = {:?}",
            (&p).inverse_truncated(len)
        );
    }
}

fn demo_rational_polynomial_inverse_truncated_invertible(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len) in rational_polynomial_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).inverse_truncated({len}) = {:?}",
            (&p).inverse_truncated(len)
        );
    }
}

fn benchmark_rational_polynomial_inverse_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.inverse_truncated(u64)",
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_pair_gen_var_4()
            .get(gm, config)
            .filter(|&(_, len)| len != 0),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len)| {
                no_out!((&p).inverse_truncated(len));
            }),
            ("basecase", &mut |(p, len)| {
                no_out!(inverse_truncated_basecase(&p, len));
            }),
            ("naive", &mut |(p, len)| {
                no_out!(inverse_truncated_naive(&p, len));
            }),
        ],
    );
}
