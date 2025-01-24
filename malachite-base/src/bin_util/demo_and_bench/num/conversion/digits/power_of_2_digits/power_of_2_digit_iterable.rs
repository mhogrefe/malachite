// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOf2DigitIterable, PowerOf2DigitIterator, PowerOf2Digits,
};
use malachite_base::test_util::bench::bucketers::{pair_1_bit_bucketer, triple_1_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_pair_gen_var_4, unsigned_triple_gen_var_3};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_power_of_2_digits);
    register_unsigned_unsigned_demos!(runner, demo_power_of_2_digits_rev);
    register_unsigned_unsigned_demos!(runner, demo_power_of_2_digits_size_hint);
    register_unsigned_unsigned_demos!(runner, demo_power_of_2_digits_get);
    register_unsigned_unsigned_benches!(runner, benchmark_power_of_2_digits_size_hint);
    register_unsigned_unsigned_benches!(runner, benchmark_power_of_2_digits_get_algorithms);
}

fn demo_power_of_2_digits<T: PowerOf2DigitIterable<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, log_base) in unsigned_pair_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}) = {:?}",
            x,
            log_base,
            PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base).collect_vec()
        );
    }
}

fn demo_power_of_2_digits_rev<
    T: PowerOf2DigitIterable<U> + PrimitiveUnsigned,
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
            "power_of_2_digits({}, {}).rev() = {:?}",
            x,
            log_base,
            PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base)
                .rev()
                .collect_vec()
        );
    }
}

fn demo_power_of_2_digits_size_hint<
    T: PowerOf2DigitIterable<U> + PrimitiveUnsigned,
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
            "power_of_2_digits({}, {}).size_hint() = {:?}",
            x,
            log_base,
            PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base).size_hint()
        );
    }
}

fn demo_power_of_2_digits_get<
    T: PowerOf2DigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, log_base, i) in unsigned_triple_gen_var_3::<T, U, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_digits({}, {}).get({}) = {:?}",
            x,
            log_base,
            i,
            PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base).get(i)
        );
    }
}

fn benchmark_power_of_2_digits_size_hint<
    T: PowerOf2DigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "PowerOf2DigitIterable::<{}>::power_of_2_digits(&{}, u64).size_hint()",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, log_base)| {
            no_out!(PowerOf2DigitIterable::<U>::power_of_2_digits(x, log_base).size_hint())
        })],
    );
}

fn benchmark_power_of_2_digits_get_algorithms<
    T: PowerOf2DigitIterable<U> + PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "PowerOf2DigitIterable::<{}>::power_of_2_digits(&{}, u64).size_hint()",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_3::<T, U, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [
            (
                &format!("power_of_2_digits({}, u64).get(u64)", T::NAME),
                &mut |(u, log_base, i)| {
                    no_out!(PowerOf2DigitIterable::<U>::power_of_2_digits(u, log_base).get(i))
                },
            ),
            (
                &format!("{}.to_power_of_2_digits_asc(u64)[usize]", T::NAME),
                &mut |(x, log_base, i)| {
                    let digits = PowerOf2Digits::<U>::to_power_of_2_digits_asc(&x, log_base);
                    let i = usize::exact_from(i);
                    if i >= digits.len() {
                        U::ZERO
                    } else {
                        digits[i]
                    };
                },
            ),
        ],
    );
}
