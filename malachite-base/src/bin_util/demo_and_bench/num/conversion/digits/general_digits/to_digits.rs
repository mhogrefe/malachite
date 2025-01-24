// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, SaturatingFrom};
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_6;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_to_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_to_digits_desc);
    register_unsigned_unsigned_benches!(runner, benchmark_to_digits_asc);
    register_unsigned_unsigned_benches!(runner, benchmark_to_digits_desc);
}

fn demo_to_digits_asc<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base) in unsigned_pair_gen_var_6::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_digits_asc({}) = {:?}",
            x,
            base,
            x.to_digits_asc(&base)
        );
    }
}

fn demo_to_digits_desc<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base) in unsigned_pair_gen_var_6::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_digits_desc({}) = {:?}",
            x,
            base,
            x.to_digits_desc(&base)
        );
    }
}

fn benchmark_to_digits_asc<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_digits_asc({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_6::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [(
            "Malachite",
            &mut |(x, base)| no_out!(x.to_digits_asc(&base)),
        )],
    );
}

fn benchmark_to_digits_desc<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_digits_desc({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_6::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base)| {
            no_out!(x.to_digits_desc(&base))
        })],
    );
}
