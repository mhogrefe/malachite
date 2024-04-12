// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::octuple_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_octuple_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_xxxx_add_yyyy_to_zzzz);
    register_unsigned_benches!(runner, benchmark_xxxx_add_yyyy_to_zzzz);
}

fn demo_xxxx_add_yyyy_to_zzzz<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0) in unsigned_octuple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "[{}, {}, {}, {}] + [{}, {}, {}, {}] = {:?}",
            x_3,
            x_2,
            x_1,
            x_0,
            y_3,
            y_2,
            y_1,
            y_0,
            T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)
        );
    }
}

fn benchmark_xxxx_add_yyyy_to_zzzz<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::xxxx_add_yyyy_to_zzzz({}, {}, {}, {}, {}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_octuple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &octuple_max_bit_bucketer("x_3", "x_2", "x_1", "x_0", "y_3", "y_2", "y_1", "y_0"),
        &mut [("default", &mut |(
            x_3,
            x_2,
            x_1,
            x_0,
            y_3,
            y_2,
            y_1,
            y_0,
        )| {
            no_out!(T::xxxx_add_yyyy_to_zzzz(
                x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0
            ))
        })],
    );
}
