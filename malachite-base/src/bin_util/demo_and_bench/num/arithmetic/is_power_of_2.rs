// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{primitive_float_bucketer, signed_bit_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_is_power_of_2);
    register_signed_demos!(runner, demo_is_power_of_2_signed);
    register_primitive_float_benches!(runner, benchmark_is_power_of_2);
    register_signed_benches!(runner, benchmark_is_power_of_2_signed);
}

fn demo_is_power_of_2<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        if f.is_power_of_2() {
            println!("{} is a power of 2", NiceFloat(f));
        } else {
            println!("{} is not a power of 2", NiceFloat(f));
        }
    }
}

fn benchmark_is_power_of_2<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_power_of_2()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(f.is_power_of_2()))],
    );
}

fn demo_is_power_of_2_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        if i.is_power_of_2() {
            println!("{i} is a power of 2");
        } else {
            println!("{i} is not a power of 2");
        }
    }
}

fn benchmark_is_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_power_of_2()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.is_power_of_2()))],
    );
}
