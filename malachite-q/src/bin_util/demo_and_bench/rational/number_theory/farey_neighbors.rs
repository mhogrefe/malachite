// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational::exhaustive::{
    exhaustive_non_negative_rationals_by_height, exhaustive_rationals_by_height,
};
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_natural_pair_gen_var_5;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_farey_neighbors);
    register_demo!(runner, demo_rational_exhaustive_by_height);
    register_demo!(runner, demo_rational_exhaustive_signed_by_height);

    register_bench!(runner, benchmark_rational_farey_neighbors);
}

fn demo_rational_farey_neighbors(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in rational_natural_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!("({x}).farey_neighbors({n}) = {:?}", x.farey_neighbors(&n));
    }
}

// The terms of the minimal-height enumerations, which the oracle steps FLINT alongside.
fn demo_rational_exhaustive_by_height(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for (i, x) in exhaustive_non_negative_rationals_by_height()
        .take(limit)
        .enumerate()
    {
        println!("minimal[{i}] = {x}");
    }
}

fn demo_rational_exhaustive_signed_by_height(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for (i, x) in exhaustive_rationals_by_height().take(limit).enumerate() {
        println!("signed_minimal[{i}] = {x}");
    }
}

fn benchmark_rational_farey_neighbors(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.farey_neighbors(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, n)| {
            no_out!(x.farey_neighbors(&n));
        })],
    );
}
