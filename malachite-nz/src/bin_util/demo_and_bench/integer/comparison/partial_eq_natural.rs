// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_2_integer_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::{integer_natural_pair_gen, integer_natural_pair_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_partial_eq_natural);
    register_demo!(runner, demo_natural_partial_eq_integer);
    register_bench!(
        runner,
        benchmark_integer_partial_eq_natural_library_comparison
    );
    register_bench!(
        runner,
        benchmark_natural_partial_eq_integer_library_comparison
    );
}

fn demo_integer_partial_eq_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

fn demo_natural_partial_eq_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if y == x {
            println!("{y} = {x}");
        } else {
            println!("{y} ≠ {x}");
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_partial_eq_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer == Natural",
        BenchmarkType::LibraryComparison,
        integer_natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_partial_eq_integer_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural == Integer",
        BenchmarkType::LibraryComparison,
        integer_natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y == x)),
            ("rug", &mut |((x, y), _)| no_out!(y == x)),
        ],
    );
}
