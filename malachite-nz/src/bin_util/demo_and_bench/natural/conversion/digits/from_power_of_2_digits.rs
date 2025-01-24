// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_10, unsigned_vec_unsigned_pair_gen_var_11,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_vec_len_times_pair_2_bucketer;
use malachite_nz::test_util::generators::{
    natural_vec_unsigned_pair_gen_var_1, natural_vec_unsigned_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_from_power_of_2_digits_asc);
    register_unsigned_demos!(runner, demo_from_power_of_2_digits_desc);
    register_unsigned_demos!(runner, demo_from_power_of_2_digits_asc_targeted);
    register_unsigned_demos!(runner, demo_from_power_of_2_digits_desc_targeted);
    register_demo!(runner, demo_natural_from_power_of_2_digits_asc_natural);
    register_demo!(runner, demo_natural_from_power_of_2_digits_desc_natural);
    register_demo!(
        runner,
        demo_natural_from_power_of_2_digits_asc_natural_targeted
    );
    register_demo!(
        runner,
        demo_natural_from_power_of_2_digits_desc_natural_targeted
    );

    register_unsigned_benches!(runner, benchmark_from_power_of_2_digits_asc_algorithms);
    register_unsigned_benches!(runner, benchmark_from_power_of_2_digits_desc);
    register_bench!(
        runner,
        benchmark_natural_from_power_of_2_digits_asc_natural_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_from_power_of_2_digits_desc_natural
    );
}

fn demo_from_power_of_2_digits_asc<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PowerOf2Digits<T>,
{
    for (digits, log_base) in unsigned_vec_unsigned_pair_gen_var_11::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_power_of_2_digits_asc({}, {:?}) = {:?}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned())
        );
    }
}

fn demo_from_power_of_2_digits_desc<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PowerOf2Digits<T>,
{
    for (digits, log_base) in unsigned_vec_unsigned_pair_gen_var_11::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_power_of_2_digits_desc({}, {:?}) = {:?}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned())
        );
    }
}

fn demo_from_power_of_2_digits_asc_targeted<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PowerOf2Digits<T>,
{
    for (digits, log_base) in unsigned_vec_unsigned_pair_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_power_of_2_digits_asc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()).unwrap()
        );
    }
}

fn demo_from_power_of_2_digits_desc_targeted<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: PowerOf2Digits<T>,
{
    for (digits, log_base) in unsigned_vec_unsigned_pair_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_power_of_2_digits_desc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned()).unwrap()
        );
    }
}

fn demo_natural_from_power_of_2_digits_asc_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (digits, log_base) in natural_vec_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural.from_power_of_2_digits_asc({}, {:?}) = {:?}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned())
        );
    }
}

fn demo_natural_from_power_of_2_digits_desc_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (digits, log_base) in natural_vec_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural.from_power_of_2_digits_desc({}, {:?}) = {:?}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned())
        );
    }
}

fn demo_natural_from_power_of_2_digits_asc_natural_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (digits, log_base) in natural_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural.from_power_of_2_digits_asc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()).unwrap()
        );
    }
}

fn demo_natural_from_power_of_2_digits_desc_natural_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (digits, log_base) in natural_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural.from_power_of_2_digits_desc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned()).unwrap()
        );
    }
}

fn benchmark_from_power_of_2_digits_asc_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T> + PowerOf2Digits<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2Digits::<Natural>::from_power_of_2_digits_asc\
                <I: Iterator<Item={}>>(I, u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("digits"),
        &mut [
            ("default", &mut |(digits, log_base)| {
                no_out!(Natural::from_power_of_2_digits_asc(
                    log_base,
                    digits.into_iter()
                ))
            }),
            ("naive", &mut |(digits, log_base)| {
                no_out!(Natural::from_power_of_2_digits_asc_naive(
                    log_base,
                    digits.into_iter()
                ))
            }),
        ],
    );
}

fn benchmark_from_power_of_2_digits_desc<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOf2Digits<T>,
{
    run_benchmark(
        &format!(
            "PowerOf2Digits::<Natural>::from_power_of_2_digits_desc\
                <I: Iterator<Item={}>>(I, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("digits"),
        &mut [("Malachite", &mut |(digits, log_base)| {
            no_out!(Natural::from_power_of_2_digits_desc(
                log_base,
                digits.into_iter()
            ))
        })],
    );
}

fn benchmark_natural_from_power_of_2_digits_asc_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_power_of_2_digits_asc<I: Iterator<Item=Natural>>(u64, I)",
        BenchmarkType::Algorithms,
        natural_vec_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_bucketer("digits", "log_base"),
        &mut [
            ("default", &mut |(digits, log_base)| {
                no_out!(Natural::from_power_of_2_digits_asc(
                    log_base,
                    digits.into_iter()
                ))
            }),
            ("naive", &mut |(digits, log_base)| {
                no_out!(Natural::from_power_of_2_digits_asc_natural_naive(
                    log_base,
                    digits.into_iter()
                ))
            }),
        ],
    );
}

fn benchmark_natural_from_power_of_2_digits_desc_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_power_of_2_digits_desc<I: Iterator<Item=Natural>>(u64, I)",
        BenchmarkType::Single,
        natural_vec_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_bucketer("digits", "log_base"),
        &mut [("Malachite", &mut |(digits, log_base)| {
            no_out!(Natural::from_power_of_2_digits_desc(
                log_base,
                digits.into_iter()
            ))
        })],
    );
}
