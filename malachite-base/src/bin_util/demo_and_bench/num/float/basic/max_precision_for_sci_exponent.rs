// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::test_util::bench::bucketers::signed_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::signed_gen_var_11;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_max_precision_for_sci_exponent);
    register_primitive_float_benches!(runner, benchmark_max_precision_for_sci_exponent);
}

fn demo_max_precision_for_sci_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for exp in signed_gen_var_11::<T>().get(gm, config).take(limit) {
        println!(
            "{}.max_precision_for_sci_exponent() = {}",
            exp,
            T::max_precision_for_sci_exponent(exp)
        );
    }
}

fn benchmark_max_precision_for_sci_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::max_precision_for_sci_exponent(i64)", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_11::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |exp| {
            no_out!(T::max_precision_for_sci_exponent(exp))
        })],
    );
}
