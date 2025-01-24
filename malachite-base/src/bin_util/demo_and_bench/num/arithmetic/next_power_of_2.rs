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
    primitive_float_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen_var_19, unsigned_gen_var_14};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_next_power_of_2_primitive_float);
    register_unsigned_demos!(runner, demo_next_power_of_2_assign_unsigned);
    register_primitive_float_demos!(runner, demo_next_power_of_2_assign_primitive_float);

    register_primitive_float_benches!(runner, benchmark_next_power_of_2_primitive_float);
    register_unsigned_benches!(runner, benchmark_next_power_of_2_assign_unsigned);
    register_primitive_float_benches!(runner, benchmark_next_power_of_2_assign_primitive_float);
}

fn demo_next_power_of_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in primitive_float_gen_var_19::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.next_power_of_2() = {}",
            NiceFloat(n),
            NiceFloat(n.next_power_of_2())
        );
    }
}

fn demo_next_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in unsigned_gen_var_14::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.next_power_of_2_assign();
        println!("n := {old_n}; n.next_power_of_2_assign(); n = {n}");
    }
}

fn demo_next_power_of_2_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in primitive_float_gen_var_19::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.next_power_of_2_assign();
        println!(
            "n := {}; n.next_power_of_2_assign(); n = {}",
            NiceFloat(old_n),
            NiceFloat(n)
        );
    }
}

fn benchmark_next_power_of_2_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.next_power_of_2()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_19::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.next_power_of_2()))],
    );
}

fn benchmark_next_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.next_power_of_2_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_14::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.next_power_of_2_assign())],
    );
}

fn benchmark_next_power_of_2_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.next_power_of_2_assign()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_19::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.next_power_of_2_assign())],
    );
}
