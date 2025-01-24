// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen_var_1};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_abs_assign_signed);
    register_primitive_float_demos!(runner, demo_abs_assign_primitive_float);
    register_signed_benches!(runner, benchmark_abs_assign_signed);
    register_primitive_float_benches!(runner, benchmark_abs_assign_primitive_float);
}

fn demo_abs_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut i in signed_gen_var_1::<T>().get(gm, config).take(limit) {
        let old_i = i;
        i.abs_assign();
        println!("i := {old_i}; i.abs_assign(); i = {i}");
    }
}

fn demo_abs_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        let old_f = f;
        f.abs_assign();
        println!(
            "i := {}; i.abs_assign(); i = {}",
            NiceFloat(old_f),
            NiceFloat(f)
        );
    }
}

fn benchmark_abs_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.abs_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.abs_assign())],
    );
}

fn benchmark_abs_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.abs_assign()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |mut f| f.abs_assign())],
    );
}
