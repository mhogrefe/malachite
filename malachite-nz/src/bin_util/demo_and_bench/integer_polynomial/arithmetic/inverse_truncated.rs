// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{InverseTruncated, Polynomial};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::inverse_truncated::*;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_1, integer_polynomial_unsigned_pair_gen_var_7,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::inverse_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_inverse_truncated);
    register_demo!(runner, demo_integer_polynomial_inverse_truncated_ref);
    register_demo!(runner, demo_integer_polynomial_inverse_truncated_unit);
    register_demo!(runner, demo_integer_polynomial_inverse_truncated_unit_ref);
    register_demo!(runner, demo_integer_polynomial_inverse_truncated_long);

    register_bench!(
        runner,
        benchmark_integer_polynomial_inverse_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_polynomial_inverse_truncated_algorithms
    );
}

fn demo_integer_polynomial_inverse_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_1()
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

fn demo_integer_polynomial_inverse_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).inverse_truncated({len}) = {:?}",
            (&p).inverse_truncated(len)
        );
    }
}

fn demo_integer_polynomial_inverse_truncated_unit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_7()
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

fn demo_integer_polynomial_inverse_truncated_unit_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len) in integer_polynomial_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).inverse_truncated({len}) = {:?}",
            (&p).inverse_truncated(len)
        );
    }
}

// The generator configuration of the long demos: polynomials of about 150 coefficients of about 8
// bits, long enough to reach the Newton and divide-and-conquer algorithms.
fn long_config(config: &GenConfig) -> GenConfig {
    let mut config = config.clone();
    if config.get_or("mean_length_n", 0) == 0 {
        config.insert("mean_length_n", 150);
    }
    if config.get_or("mean_bits_n", 0) == 0 {
        config.insert("mean_bits_n", 8);
    }
    config
}

// Inverses of long series, truncated to 16 more coefficients than the series has.
fn demo_integer_polynomial_inverse_truncated_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _) in integer_polynomial_unsigned_pair_gen_var_7()
        .get(gm, &long_config(config))
        .take(limit)
    {
        let len = p.len() + 16;
        println!(
            "(&({p})).inverse_truncated({len}) = {:?}",
            (&p).inverse_truncated(len)
        );
    }
}

fn benchmark_integer_polynomial_inverse_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.inverse_truncated(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            (
                "IntegerPolynomial.inverse_truncated(u64)",
                &mut |(p, len)| {
                    no_out!(p.inverse_truncated(len));
                },
            ),
            (
                "(&IntegerPolynomial).inverse_truncated(u64)",
                &mut |(p, len)| {
                    no_out!((&p).inverse_truncated(len));
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_inverse_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.inverse_truncated(u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_7()
            .get(gm, config)
            .filter(|&(_, len)| len != 0),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len)| {
                no_out!((&p).inverse_truncated(len));
            }),
            ("basecase", &mut |(p, len)| {
                no_out!(inverse_truncated_basecase(
                    p.coefficients_asc(),
                    usize::exact_from(len)
                ));
            }),
            ("Newton", &mut |(p, len)| {
                no_out!(inverse_truncated_newton(
                    p.coefficients_asc(),
                    usize::exact_from(len)
                ));
            }),
            ("naive", &mut |(p, len)| {
                no_out!(inverse_truncated_naive(&p, len));
            }),
        ],
    );
}
