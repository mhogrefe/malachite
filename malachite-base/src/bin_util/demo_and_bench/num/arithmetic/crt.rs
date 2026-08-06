// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_13;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_crt);
    register_unsigned_benches!(runner, benchmark_crt);
}

fn demo_crt<T: Crt<T, T, T, Output = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (r1, m1, r2, m2) in unsigned_quadruple_gen_var_13::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{r1}.crt({m1}, {r2}, {m2}) = {:?}", r1.crt(m1, r2, m2));
    }
}

fn benchmark_crt<T: Crt<T, T, T, Output = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.crt({}, {}, {})", T::NAME, T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_quadruple_gen_var_13::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("r1", "m1", "r2", "m2"),
        &mut [("Malachite", &mut |(r1, m1, r2, m2)| {
            no_out!(r1.crt(m1, r2, m2));
        })],
    );
}
