// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::string_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, string_gen_var_10};
use malachite_base::test_util::runner::Runner;
use std::fmt::Debug;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_nice_float_from_str);
    register_primitive_float_demos!(runner, demo_nice_float_from_str_targeted);
    register_primitive_float_benches!(runner, benchmark_nice_float_from_str);
}

fn demo_nice_float_from_str<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    <T as FromStr>::Err: Debug,
{
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "NiceFloat::from_str({:?}) = {:?}",
            s,
            NiceFloat::<T>::from_str(&s)
        );
    }
}

fn demo_nice_float_from_str_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    <T as FromStr>::Err: Debug,
{
    for s in string_gen_var_10().get(gm, config).take(limit) {
        println!(
            "NiceFloat::from_str({:?}) = {:?}",
            s,
            NiceFloat::<T>::from_str(&s)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_nice_float_from_str<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("NiceFloat::<{}>::from_str(&str)", T::NAME),
        BenchmarkType::Single,
        string_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(NiceFloat::<T>::from_str(&s)))],
    );
}
