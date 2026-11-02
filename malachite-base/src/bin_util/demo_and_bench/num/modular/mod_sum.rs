// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_sum);
    register_unsigned_benches!(runner, benchmark_mod_sum_algorithms);
}

fn demo_mod_sum<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in unsigned_vector_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.mod_sum({v}, {m}) = {}",
            T::NAME,
            T::mod_sum(v.elements.iter().copied(), m)
        );
    }
}

fn benchmark_mod_sum_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::mod_sum(Iterator<Item = {}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("xs"),
        &mut [
            ("default", &mut |(v, m)| {
                no_out!(T::mod_sum(v.elements.into_iter(), m));
            }),
            ("fold", &mut |(v, m)| {
                no_out!(v.elements.into_iter().fold(T::ZERO, |s, x| s.mod_add(x, m)));
            }),
        ],
    );
}
