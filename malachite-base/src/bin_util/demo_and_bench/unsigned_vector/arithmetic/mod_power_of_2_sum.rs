// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2Sum};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_sum);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_sum_algorithms
    );
}

fn demo_unsigned_vector_mod_power_of_2_sum<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, w, pow) in
        unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "UnsignedVector::mod_power_of_2_sum([{u}, {v}, {w}], {pow}) = {}",
            UnsignedVector::mod_power_of_2_sum([&u, &v, &w].into_iter(), pow)
        );
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_sum_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>::mod_power_of_2_sum(Iterator<Item = UnsignedVector<{}>>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_vector_dimension_bucketer("u"),
        &mut [
            ("default", &mut |(u, v, w, pow)| {
                no_out!(UnsignedVector::mod_power_of_2_sum(
                    [u, v, w].into_iter(),
                    pow
                ));
            }),
            ("repeated addition", &mut |(u, v, w, pow)| {
                no_out!(u.mod_power_of_2_add(v, pow).mod_power_of_2_add(w, pow));
            }),
        ],
    );
}
