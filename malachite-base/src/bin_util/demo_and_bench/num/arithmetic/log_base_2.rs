// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    primitive_float_bucketer, primitive_int_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen_var_18, unsigned_gen_var_1};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_log_base_2_unsigned);
    register_unsigned_demos!(runner, demo_ceiling_log_base_2_unsigned);
    register_unsigned_demos!(runner, demo_checked_log_base_2_unsigned);
    register_primitive_float_demos!(runner, demo_floor_log_base_2_primitive_float);
    register_primitive_float_demos!(runner, demo_ceiling_log_base_2_primitive_float);
    register_primitive_float_demos!(runner, demo_checked_log_base_2_primitive_float);
    register_unsigned_benches!(runner, benchmark_floor_log_base_2_unsigned);
    register_unsigned_benches!(runner, benchmark_ceiling_log_base_2_unsigned);
    register_unsigned_benches!(runner, benchmark_checked_log_base_2_unsigned);
    register_primitive_float_benches!(runner, benchmark_floor_log_base_2_primitive_float);
    register_primitive_float_benches!(runner, benchmark_ceiling_log_base_2_primitive_float);
    register_primitive_float_benches!(runner, benchmark_checked_log_base_2_primitive_float);
}

fn demo_floor_log_base_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("{}.floor_log_base_2() = {}", n, n.floor_log_base_2());
    }
}

fn demo_ceiling_log_base_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("{}.ceiling_log_base_2() = {}", n, n.ceiling_log_base_2());
    }
}

fn demo_checked_log_base_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("{}.checked_log_base_2() = {:?}", n, n.checked_log_base_2());
    }
}

fn demo_floor_log_base_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in primitive_float_gen_var_18::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.floor_log_base_2() = {}",
            NiceFloat(n),
            n.floor_log_base_2()
        );
    }
}

fn demo_ceiling_log_base_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in primitive_float_gen_var_18::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.ceiling_log_base_2() = {}",
            NiceFloat(n),
            n.ceiling_log_base_2()
        );
    }
}

fn demo_checked_log_base_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in primitive_float_gen_var_18::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.checked_log_base_2() = {:?}",
            NiceFloat(n),
            n.checked_log_base_2()
        );
    }
}

fn benchmark_floor_log_base_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_log_base_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2()))],
    );
}

fn benchmark_ceiling_log_base_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_log_base_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2()))],
    );
}

fn benchmark_checked_log_base_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_log_base_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.checked_log_base_2()))],
    );
}

fn benchmark_floor_log_base_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_log_base_2()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_18::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2()))],
    );
}

fn benchmark_ceiling_log_base_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_log_base_2()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_18::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2()))],
    );
}

fn benchmark_checked_log_base_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_log_base_2()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_18::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.checked_log_base_2()))],
    );
}
