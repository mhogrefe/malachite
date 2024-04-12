// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{PowerOf2DigitIterable, PowerOf2Digits};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_6, natural_unsigned_pair_gen_var_7,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_to_power_of_2_digits_asc);
    register_unsigned_demos!(runner, demo_to_power_of_2_digits_desc);
    register_demo!(runner, demo_natural_to_power_of_2_digits_asc_natural);
    register_demo!(runner, demo_natural_to_power_of_2_digits_desc_natural);

    register_unsigned_benches!(runner, benchmark_to_power_of_2_digits_asc_algorithms);
    register_unsigned_benches!(
        runner,
        benchmark_to_power_of_2_digits_asc_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_to_power_of_2_digits_desc_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_to_power_of_2_digits_asc_natural_algorithms
    );
    register_bench!(runner, benchmark_natural_to_power_of_2_digits_desc_natural);
}

fn demo_to_power_of_2_digits_asc<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PowerOf2Digits<T>,
{
    for (n, log_base) in natural_unsigned_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_2_digits_asc({}) = {:?}",
            n,
            log_base,
            PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base)
        );
    }
}

fn demo_to_power_of_2_digits_desc<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PowerOf2Digits<T>,
{
    for (n, log_base) in natural_unsigned_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_2_digits_desc({}) = {:?}",
            n,
            log_base,
            PowerOf2Digits::<T>::to_power_of_2_digits_desc(&n, log_base)
        );
    }
}

fn demo_natural_to_power_of_2_digits_asc_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_2_digits_asc({}) = {:?}",
            n,
            log_base,
            PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, log_base)
        );
    }
}

fn demo_natural_to_power_of_2_digits_desc_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_2_digits_desc({}) = {:?}",
            n,
            log_base,
            PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&n, log_base)
        );
    }
}

fn benchmark_to_power_of_2_digits_asc_algorithms<
    T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOf2Digits<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2Digits::<{}>::to_power_of_2_digits_asc(&Natural, u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, log_base)| {
                no_out!(PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base))
            }),
            ("naive", &mut |(n, log_base)| {
                no_out!(Natural::to_power_of_2_digits_asc_naive::<T>(&n, log_base))
            }),
            ("using iterator", &mut |(n, log_base)| {
                no_out!(
                    PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base).collect_vec()
                )
            }),
        ],
    );
}

fn benchmark_to_power_of_2_digits_asc_evaluation_strategy<
    T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOf2Digits<T>,
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2Digits::<{}>::to_power_of_2_digits_asc(&Natural, u64)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, log_base)| {
                no_out!(PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base))
            }),
            (
                "Natural.power_of_2_digits(u64).collect_vec()",
                &mut |(n, log_base)| {
                    no_out!(
                        PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base).collect_vec()
                    )
                },
            ),
        ],
    );
}

fn benchmark_to_power_of_2_digits_desc_evaluation_strategy<
    T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOf2Digits<T>,
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2Digits::<{}>::to_power_of_2_digits_desc(&Natural, u64)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, log_base)| {
                no_out!(PowerOf2Digits::<T>::to_power_of_2_digits_desc(&n, log_base))
            }),
            (
                "Natural.power_of_2_digits(u64).rev().collect_vec()",
                &mut |(n, log_base)| {
                    no_out!(PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base)
                        .rev()
                        .collect_vec())
                },
            ),
        ],
    );
}

fn benchmark_natural_to_power_of_2_digits_asc_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural, u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, log_base)| {
                no_out!(PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(
                    &n, log_base
                ))
            }),
            ("naive", &mut |(n, log_base)| {
                no_out!(n.to_power_of_2_digits_asc_natural_naive(log_base))
            }),
            ("using iterator", &mut |(n, log_base)| {
                no_out!(
                    PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base)
                        .rev()
                        .collect_vec()
                )
            }),
        ],
    );
}

fn benchmark_natural_to_power_of_2_digits_desc_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural, u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, log_base)| {
            no_out!(PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(
                &n, log_base
            ))
        })],
    );
}
