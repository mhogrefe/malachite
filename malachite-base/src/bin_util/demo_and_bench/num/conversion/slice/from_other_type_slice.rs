// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::FromOtherTypeSlice;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use std::fmt::Display;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_from_other_type_slice);
    register_unsigned_unsigned_benches!(runner, benchmark_from_other_type_slice);
}

fn demo_from_other_type_slice<T: Display + FromOtherTypeSlice<U> + Named, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for xs in unsigned_vec_gen::<U>().get(gm, config).take(limit) {
        println!(
            "{}::from_other_type_slice({:?}) = {}",
            T::NAME,
            xs,
            T::from_other_type_slice(&xs)
        );
    }
}

fn benchmark_from_other_type_slice<T: FromOtherTypeSlice<U> + Named, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.from_other_type_slice(&[{}])", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_vec_gen::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(T::from_other_type_slice(&xs))
        })],
    );
}
