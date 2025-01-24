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
use malachite_nz::test_util::bench::bucketers::rational_from_power_of_2_digits_bucketer;
use malachite_nz::test_util::generators::{large_type_gen_var_23, large_type_gen_var_24};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_power_of_2_digits);
    register_demo!(runner, demo_rational_from_power_of_2_digits_ref);
    register_demo!(runner, demo_rational_from_power_of_2_digits_binary);
    register_bench!(
        runner,
        benchmark_rational_from_power_of_2_digits_evaluation_strategy
    );
}

fn demo_rational_from_power_of_2_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (log_base, before_point, after_point) in large_type_gen_var_23().get(gm, config).take(limit)
    {
        println!(
            "from_power_of_2_digits({}, {:?}, {}) = {}",
            log_base,
            before_point.clone(),
            after_point.clone(),
            Rational::from_power_of_2_digits(log_base, before_point, after_point)
        );
    }
}

fn demo_rational_from_power_of_2_digits_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (log_base, before_point, after_point) in large_type_gen_var_23().get(gm, config).take(limit)
    {
        println!(
            "from_power_of_2_digits_ref({}, {:?}, {:?}) = {}",
            log_base,
            before_point,
            after_point,
            Rational::from_power_of_2_digits_ref(log_base, &before_point, &after_point)
        );
    }
}

fn demo_rational_from_power_of_2_digits_binary(gm: GenMode, config: &GenConfig, limit: usize) {
    for (before_point, after_point) in large_type_gen_var_24().get(gm, config).take(limit) {
        println!(
            "from_power_of_2_digits(1, {:?}, {}) = {}",
            before_point.clone(),
            after_point.clone(),
            Rational::from_power_of_2_digits(1, before_point, after_point)
        );
    }
}

fn benchmark_rational_from_power_of_2_digits_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_power_of_2_digits(u64, &[Natural], &RationalSequence<Natural>)",
        BenchmarkType::EvaluationStrategy,
        large_type_gen_var_23().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_from_power_of_2_digits_bucketer(),
        &mut [
            (
                "Rational::from_power_of_2_digits(u64, Vec<Natural>, RationalSequence<Natural>)",
                &mut |(log_base, before_point, after_point)| {
                    no_out!(Rational::from_power_of_2_digits(
                        log_base,
                        before_point,
                        after_point
                    ))
                },
            ),
            (
                "Rational::from_power_of_2_digits_ref(u64, &[Natural], &RationalSequence<Natural>)",
                &mut |(log_base, before_point, after_point)| {
                    no_out!(Rational::from_power_of_2_digits_ref(
                        log_base,
                        &before_point,
                        &after_point
                    ))
                },
            ),
        ],
    );
}
