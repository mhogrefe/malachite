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
    unsigned_vec_unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_3,
    unsigned_vec_unsigned_pair_gen_var_6,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_from_power_of_2_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_from_power_of_2_digits_desc);
    register_unsigned_unsigned_demos!(runner, demo_from_power_of_2_digits_asc_targeted);
    register_unsigned_unsigned_demos!(runner, demo_from_power_of_2_digits_desc_targeted);
    register_unsigned_unsigned_benches!(runner, benchmark_from_power_of_2_digits_asc);
    register_unsigned_unsigned_benches!(runner, benchmark_from_power_of_2_digits_desc);
}

fn demo_from_power_of_2_digits_asc<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, log_base) in unsigned_vec_unsigned_pair_gen_var_6::<U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_power_of_2_digits_asc({}, {:?}) = {:?}",
            T::NAME,
            log_base,
            xs,
            T::from_power_of_2_digits_asc(log_base, xs.iter().cloned())
        );
    }
}

fn demo_from_power_of_2_digits_desc<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, log_base) in unsigned_vec_unsigned_pair_gen_var_6::<U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_power_of_2_digits_desc({}, {:?}) = {:?}",
            T::NAME,
            log_base,
            xs,
            T::from_power_of_2_digits_desc(log_base, xs.iter().cloned())
        );
    }
}

fn demo_from_power_of_2_digits_asc_targeted<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, log_base) in unsigned_vec_unsigned_pair_gen_var_2::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_power_of_2_digits_asc({}, {:?}) = {}",
            T::NAME,
            log_base,
            xs,
            T::from_power_of_2_digits_asc(log_base, xs.iter().cloned()).unwrap()
        );
    }
}

fn demo_from_power_of_2_digits_desc_targeted<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, log_base) in unsigned_vec_unsigned_pair_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_power_of_2_digits_desc({}, {:?}) = {}",
            T::NAME,
            log_base,
            xs,
            T::from_power_of_2_digits_desc(log_base, xs.iter().cloned()).unwrap()
        );
    }
}

fn benchmark_from_power_of_2_digits_asc<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_power_of_2_digits_asc<I: Iterator<Item={}>>(u64, I)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_2::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, log_base)| {
            no_out!(T::from_power_of_2_digits_asc(log_base, xs.into_iter()))
        })],
    );
}

fn benchmark_from_power_of_2_digits_desc<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_power_of_2_digits_asc<I: Iterator<Item={}>>(u64, I)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_3::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, log_base)| {
            no_out!(T::from_power_of_2_digits_desc(log_base, xs.into_iter()))
        })],
    );
}
