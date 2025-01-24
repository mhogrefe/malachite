// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_vec_natural_sum_bits_bucketer;
use malachite_nz::test_util::generators::natural_vec_integer_pair_gen_var_1;
use malachite_q::test_util::conversion::continued_fraction::from_continued_fraction::*;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_continued_fraction);
    register_demo!(runner, demo_rational_from_continued_fraction_ref);
    register_bench!(
        runner,
        benchmark_rational_from_continued_fraction_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_from_continued_fraction_evaluation_strategy
    );
}

fn demo_rational_from_continued_fraction(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, floor) in natural_vec_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_continued_fraction({}, {:?}) = {}",
            floor.clone(),
            xs.clone(),
            Rational::from_continued_fraction(floor, xs.into_iter())
        );
    }
}

fn demo_rational_from_continued_fraction_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, floor) in natural_vec_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_continued_fraction_ref({}, {:?}) = {}",
            floor,
            xs,
            Rational::from_continued_fraction_ref(&floor, xs.iter())
        );
    }
}

fn benchmark_rational_from_continued_fraction_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_continued_fraction(&Integer, &[Natural])",
        BenchmarkType::Algorithms,
        natural_vec_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_natural_sum_bits_bucketer(),
        &mut [
            ("default", &mut |(xs, floor)| {
                no_out!(Rational::from_continued_fraction(floor, xs.into_iter()))
            }),
            ("alt", &mut |(xs, floor)| {
                no_out!(from_continued_fraction_alt(floor, xs))
            }),
        ],
    );
}

fn benchmark_rational_from_continued_fraction_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_continued_fraction(&Integer, &[Natural])",
        BenchmarkType::EvaluationStrategy,
        natural_vec_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_natural_sum_bits_bucketer(),
        &mut [
            (
                "Rational::from_continued_fraction(Integer, Vec<Natural>)",
                &mut |(xs, floor)| {
                    no_out!(Rational::from_continued_fraction(floor, xs.into_iter()))
                },
            ),
            (
                "Rational::from_continued_fraction_ref(&Integer, &[Natural])",
                &mut |(xs, floor)| {
                    no_out!(Rational::from_continued_fraction_ref(&floor, xs.iter()))
                },
            ),
        ],
    );
}
