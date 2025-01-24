// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOf2DigitIterable, PowerOf2DigitIterator, PowerOf2Digits,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_bit_bucketer, triple_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_6, natural_unsigned_pair_gen_var_7,
    natural_unsigned_unsigned_triple_gen_var_2, natural_unsigned_unsigned_triple_gen_var_3,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_power_of_2_digits);
    register_unsigned_demos!(runner, demo_natural_power_of_2_digits_rev);
    register_unsigned_demos!(runner, demo_natural_power_of_2_digits_size_hint);
    register_unsigned_demos!(runner, demo_natural_power_of_2_digits_get);
    register_demo!(runner, demo_natural_power_of_2_digits_natural);
    register_demo!(runner, demo_natural_power_of_2_digits_rev_natural);
    register_demo!(runner, demo_natural_power_of_2_digits_size_hint_natural);
    register_demo!(runner, demo_natural_power_of_2_digits_get_natural);

    register_unsigned_benches!(runner, benchmark_natural_power_of_2_digits_size_hint);
    register_unsigned_benches!(runner, benchmark_natural_power_of_2_digits_get_algorithms);
    register_bench!(
        runner,
        benchmark_natural_power_of_2_digits_size_hint_natural
    );
    register_bench!(
        runner,
        benchmark_natural_power_of_2_digits_get_natural_algorithms
    );
}

fn demo_natural_power_of_2_digits<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    for (n, log_base) in natural_unsigned_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}) = {:?}",
            n,
            log_base,
            PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base).collect_vec()
        );
    }
}

fn demo_natural_power_of_2_digits_rev<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    for (n, log_base) in natural_unsigned_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).rev() = {:?}",
            n,
            log_base,
            PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base)
                .rev()
                .collect_vec()
        );
    }
}

fn demo_natural_power_of_2_digits_size_hint<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    for (n, log_base) in natural_unsigned_pair_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).size_hint() = {:?}",
            n,
            log_base,
            PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base).size_hint()
        );
    }
}

fn demo_natural_power_of_2_digits_get<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    for (n, log_base, i) in natural_unsigned_unsigned_triple_gen_var_2::<u64, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).get({}) = {:?}",
            n,
            log_base,
            i,
            PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base).get(i)
        );
    }
}

fn demo_natural_power_of_2_digits_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}) = {:?}",
            n,
            log_base,
            PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base).collect_vec()
        );
    }
}

fn demo_natural_power_of_2_digits_rev_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).rev() = {:?}",
            n,
            log_base,
            PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base)
                .rev()
                .collect_vec()
        );
    }
}

fn demo_natural_power_of_2_digits_size_hint_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).size_hint() = {:?}",
            n,
            log_base,
            PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base).size_hint()
        );
    }
}

fn demo_natural_power_of_2_digits_get_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base, i) in natural_unsigned_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).get({}) = {:?}",
            n,
            log_base,
            i,
            PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base).get(i)
        );
    }
}

fn benchmark_natural_power_of_2_digits_size_hint<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2DigitIterable::<{}>::power_of_2_digits(&Natural, u64).size_hint()",
            T::NAME
        ),
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [(
            &format!(
                "PowerOf2DigitIterable::<{}>::power_of_2_digits(&Natural, u64).size_hint()",
                T::NAME
            ),
            &mut |(n, log_base)| {
                no_out!(PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base).size_hint())
            },
        )],
    );
}

fn benchmark_natural_power_of_2_digits_get_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
    Natural: PowerOf2Digits<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2DigitIterable::<{}>::power_of_2_digits(&Natural, u64).get(u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        natural_unsigned_unsigned_triple_gen_var_2::<u64, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            (
                "power_of_2_digits(&Natural, u64).get(u64)",
                &mut |(n, log_base, i)| {
                    no_out!(PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base).get(i))
                },
            ),
            (
                "Natural.to_power_of_2_digits_asc(u64)[u64]",
                &mut |(n, log_base, i)| {
                    let digits = PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base);
                    let i = usize::exact_from(i);
                    if i >= digits.len() {
                        T::ZERO
                    } else {
                        digits[i]
                    };
                },
            ),
        ],
    );
}

fn benchmark_natural_power_of_2_digits_size_hint_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "PowerOf2DigitIterable::<Natural>::power_of_2_digits(&Natural, u64).size_hint()",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [(
            "PowerOf2DigitIterable::<Natural>::power_of_2_digits(&Natural, u64).size_hint()",
            &mut |(n, log_base)| {
                no_out!(
                    PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base).size_hint()
                )
            },
        )],
    );
}

#[allow(clippy::let_unit_value)]
fn benchmark_natural_power_of_2_digits_get_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "PowerOf2DigitIterable::<Natural>::power_of_2_digits(&Natural, u64).get(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            (
                "power_of_2_digits(&Natural, u64).get(u64)",
                &mut |(n, log_base, i)| {
                    no_out!(
                        PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base).get(i)
                    )
                },
            ),
            (
                "Natural.to_power_of_2_digits_asc(u64)[u64]",
                &mut |(n, log_base, i)| {
                    let digits = PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, log_base);
                    let i = usize::exact_from(i);
                    let _result = if i >= digits.len() {
                        let _ = Natural::ZERO;
                    } else {
                        let _ = digits[i];
                    };
                },
            ),
        ],
    );
}
