// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::{OrdAbs, OrdAbsDouble, OrdDouble};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_integer_max_bit_bucketer, pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen, natural_pair_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_cmp_double);
    register_demo!(runner, demo_integer_cmp_abs_double);

    register_bench!(runner, benchmark_natural_cmp_double_algorithms);
    register_bench!(runner, benchmark_integer_cmp_abs_double_algorithms);
}

fn demo_natural_cmp_double(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("{}.cmp_double(&{}) = {:?}", x, y, x.cmp_double(&y));
    }
}

fn demo_integer_cmp_abs_double(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        println!(
            "({}).cmp_abs_double(&{}) = {:?}",
            x,
            y,
            x.cmp_abs_double(&y)
        );
    }
}

fn benchmark_natural_cmp_double_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.cmp_double(&Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("no allocation", &mut |(x, y)| no_out!(x.cmp_double(&y))),
            ("doubling first", &mut |(x, y)| {
                let _ = x.cmp(&(&y << 1u64));
            }),
        ],
    );
}

fn benchmark_integer_cmp_abs_double_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.cmp_abs_double(&Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("no allocation", &mut |(x, y)| no_out!(x.cmp_abs_double(&y))),
            ("doubling first", &mut |(x, y)| {
                no_out!(x.cmp_abs(&(&y << 1u64)));
            }),
        ],
    );
}
