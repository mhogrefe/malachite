// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::xx_sub_yy_to_zz::explicit_xx_sub_yy_to_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_10;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_xx_sub_yy_to_zz);
    register_unsigned_benches!(runner, benchmark_xx_sub_yy_to_zz_algorithms);
}

fn demo_xx_sub_yy_to_zz<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x_1, x_0, y_1, y_0) in unsigned_quadruple_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "[{}, {}] - [{}, {}] = {:?}",
            x_1,
            x_0,
            y_1,
            y_0,
            T::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0)
        );
    }
}

fn benchmark_xx_sub_yy_to_zz_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::xx_sub_yy_to_zz({}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("x_1", "x_0", "y_1", "y_0"),
        &mut [
            ("default", &mut |(x_1, x_0, y_1, y_0)| {
                no_out!(T::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0))
            }),
            ("explicit", &mut |(x_1, x_0, y_1, y_0)| {
                no_out!(explicit_xx_sub_yy_to_zz(x_1, x_0, y_1, y_0))
            }),
        ],
    );
}
