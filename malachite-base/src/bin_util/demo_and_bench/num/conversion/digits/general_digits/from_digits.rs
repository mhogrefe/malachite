// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, SaturatingFrom};
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_7, unsigned_vec_unsigned_pair_gen_var_8,
    unsigned_vec_unsigned_pair_gen_var_9,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_from_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_from_digits_desc);
    register_unsigned_unsigned_demos!(runner, demo_from_digits_asc_targeted);
    register_unsigned_unsigned_demos!(runner, demo_from_digits_desc_targeted);
    register_unsigned_unsigned_benches!(runner, benchmark_from_digits_asc);
    register_unsigned_unsigned_benches!(runner, benchmark_from_digits_desc);
}

fn demo_from_digits_asc<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_9::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.from_digits_asc({}, {:?}) = {:?}",
            U::NAME,
            base,
            xs,
            U::from_digits_asc(&base, xs.iter().copied())
        );
    }
}

fn demo_from_digits_desc<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_9::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.from_digits_desc({}, {:?}) = {:?}",
            U::NAME,
            base,
            xs,
            U::from_digits_desc(&base, xs.iter().copied())
        );
    }
}

fn demo_from_digits_asc_targeted<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_8::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.from_digits_asc({}, {:?}) = {}",
            U::NAME,
            base,
            xs,
            U::from_digits_asc(&base, xs.iter().copied()).unwrap()
        );
    }
}

fn demo_from_digits_desc_targeted<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_7::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.from_digits_desc({}, {:?}) = {}",
            U::NAME,
            base,
            xs,
            U::from_digits_desc(&base, xs.iter().copied()).unwrap()
        );
    }
}

fn benchmark_from_digits_asc<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.from_digits_asc({}, Iterator<Item={}>)",
            U::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_8::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("digits"),
        &mut [("Malachite", &mut |(xs, base)| {
            no_out!(U::from_digits_asc(&base, xs.iter().copied()))
        })],
    );
}

fn benchmark_from_digits_desc<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.from_digits_desc({}, Iterator<Item={}>)",
            U::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_7::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("digits"),
        &mut [("Malachite", &mut |(xs, base)| {
            no_out!(U::from_digits_desc(&base, xs.iter().copied()))
        })],
    );
}
