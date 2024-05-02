// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::quadruple_1_rational_sequence_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::large_type_gen_var_22;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_mutate);
    register_bench!(runner, benchmark_rational_sequence_mutate);
}

fn demo_rational_sequence_mutate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, index, y, z) in large_type_gen_var_22::<u8>().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let out = xs.mutate(index, |x| {
            *x = y;
            z
        });
        println!("xs := {xs_old}; xs.mutate({index}, |x| {{ *x = {y}; {z} }}) = {out}; xs = {xs}");
    }
}

fn benchmark_rational_sequence_mutate(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.mutate(usize, FnOnce(&mut T) -> U)",
        BenchmarkType::Single,
        large_type_gen_var_22::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, index, y, z)| {
            no_out!(xs.mutate(index, |x| {
                *x = y;
                z
            }))
        })],
    );
}
