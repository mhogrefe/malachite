// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::test_util::bench::bucketers::pair_vec_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen_var_7;
use malachite_q::test_util::rational::conversion::digits::factoradic_digits::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_factoradic_digits);
    register_demo!(runner, demo_rational_from_factoradic_digits);
    register_bench!(runner, benchmark_rational_factoradic_digits_algorithms);
    register_bench!(runner, benchmark_rational_from_factoradic_digits_algorithms);
}

fn demo_rational_factoradic_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_7().get(gm, config).take(limit) {
        let (before_point, after_point) = x.factoradic_digits();
        println!(
            "({}).factoradic_digits() = ({:?}, {:?})",
            x,
            before_point,
            after_point.collect_vec()
        );
    }
}

fn demo_rational_from_factoradic_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_7().get(gm, config).take(limit) {
        let (before_point, after_point) = x.factoradic_digits();
        let after_point = after_point.collect_vec();
        println!(
            "Rational::from_factoradic_digits({:?}, {:?}) = {:?}",
            before_point,
            after_point,
            Rational::from_factoradic_digits(&before_point, &after_point)
        );
    }
}

fn benchmark_rational_factoradic_digits_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.factoradic_digits()",
        BenchmarkType::Algorithms,
        rational_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                let (before_point, after_point) = x.factoradic_digits();
                no_out!((before_point, after_point.collect_vec()));
            }),
            ("naive", &mut |x| {
                no_out!(factoradic_digits_naive(&x));
            }),
        ],
    );
}

fn benchmark_rational_from_factoradic_digits_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_factoradic_digits(&[u64], &[u64])",
        BenchmarkType::Algorithms,
        rational_gen_var_7().get(gm, config).map(|x| {
            let (before_point, after_point) = x.factoradic_digits();
            (before_point, after_point.collect_vec())
        }),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("before_point", "after_point"),
        &mut [
            ("default", &mut |(xs, ys)| {
                no_out!(Rational::from_factoradic_digits(&xs, &ys));
            }),
            ("naive", &mut |(xs, ys)| {
                no_out!(from_factoradic_digits_naive(&xs, &ys));
            }),
        ],
    );
}
