// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{signed_abs_bucketer, unsigned_direct_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen_var_11, unsigned_gen_var_15, unsigned_gen_var_16,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_power_of_2_unsigned);
    register_signed_demos!(runner, demo_power_of_2_signed);
    register_primitive_float_demos!(runner, demo_power_of_2_primitive_float);
    register_unsigned_benches!(runner, benchmark_power_of_2_unsigned);
    register_signed_benches!(runner, benchmark_power_of_2_signed);
    register_primitive_float_benches!(runner, benchmark_power_of_2_primitive_float);
}

fn demo_power_of_2_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for pow in unsigned_gen_var_15::<T>().get(gm, config).take(limit) {
        println!("2^{} = {}", pow, T::power_of_2(pow));
    }
}

fn demo_power_of_2_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for pow in unsigned_gen_var_16::<T>().get(gm, config).take(limit) {
        println!("2^{} = {}", pow, T::power_of_2(pow));
    }
}

fn demo_power_of_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for pow in signed_gen_var_11::<T>().get(gm, config).take(limit) {
        println!("2^({}) = {}", pow, NiceFloat(T::power_of_2(pow)));
    }
}

fn benchmark_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_15::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |pow| no_out!(T::power_of_2(pow)))],
    );
}

fn benchmark_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_16::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |pow| no_out!(T::power_of_2(pow)))],
    );
}

fn benchmark_power_of_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.power_of_2(i64)", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_11::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_abs_bucketer("pow"),
        &mut [("Malachite", &mut |pow| no_out!(T::power_of_2(pow)))],
    );
}
