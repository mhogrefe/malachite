// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{PowerOf2DigitIterable, PowerOf2Digits};
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_4;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_to_power_of_2_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_to_power_of_2_digits_desc);
    register_unsigned_unsigned_benches!(
        runner,
        benchmark_to_power_of_2_digits_asc_evaluation_strategy
    );
    register_unsigned_unsigned_benches!(
        runner,
        benchmark_to_power_of_2_digits_desc_evaluation_strategy
    );
}

fn demo_to_power_of_2_digits_asc<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, log_base) in unsigned_pair_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_2_digits_asc({}) = {:?}",
            x,
            log_base,
            PowerOf2Digits::<U>::to_power_of_2_digits_asc(&x, log_base)
        );
    }
}

fn demo_to_power_of_2_digits_desc<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, log_base) in unsigned_pair_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_2_digits_desc({}) = {:?}",
            x,
            log_base,
            PowerOf2Digits::<U>::to_power_of_2_digits_desc(&x, log_base)
        );
    }
}

fn benchmark_to_power_of_2_digits_asc_evaluation_strategy<
    T: PowerOf2Digits<U> + PowerOf2DigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "PowerOf2Digits::<{}>::to_power_of_2_digits_asc({}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_pair_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(x, log_base)| {
                no_out!(PowerOf2Digits::<U>::to_power_of_2_digits_asc(&x, log_base))
            }),
            (
                &format!("{}.power_of_2_digits(u64).collect_vec()", T::NAME),
                &mut |(x, log_base)| {
                    PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base).collect_vec();
                },
            ),
        ],
    );
}

fn benchmark_to_power_of_2_digits_desc_evaluation_strategy<
    T: PowerOf2Digits<U> + PowerOf2DigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "PowerOf2Digits::<{}>::to_power_of_2_digits_desc({}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_pair_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(x, log_base)| {
                no_out!(PowerOf2Digits::<U>::to_power_of_2_digits_desc(&x, log_base))
            }),
            (
                &format!("{}.power_of_2_digits(u64).rev().collect_vec()", T::NAME),
                &mut |(x, log_base)| {
                    no_out!(PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base)
                        .rev()
                        .collect_vec())
                },
            ),
        ],
    );
}
