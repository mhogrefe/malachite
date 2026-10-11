// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModSum};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_sum);
    register_unsigned_benches!(runner, benchmark_unsigned_vector_mod_sum_algorithms);
}

fn demo_unsigned_vector_mod_sum<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, w, m) in
        unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2::<T>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "UnsignedVector::mod_sum([{u}, {v}, {w}], {m}) = {}",
            UnsignedVector::mod_sum([&u, &v, &w].into_iter(), m)
        );
    }
}

fn benchmark_unsigned_vector_mod_sum_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>::mod_sum(Iterator<Item = UnsignedVector<{}>>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2::<T>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_vector_dimension_bucketer("u"),
        &mut [
            ("default", &mut |(u, v, w, m)| {
                no_out!(UnsignedVector::mod_sum([u, v, w].into_iter(), m));
            }),
            ("repeated addition", &mut |(u, v, w, m)| {
                no_out!(u.mod_add(v, m).mod_add(w, m));
            }),
        ],
    );
}
