// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSqrt;
use malachite_base::num::factorization::traits::IsSquare;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_is_square);
    register_bench!(runner, benchmark_natural_is_square_library_comparison);
    register_bench!(runner, benchmark_natural_is_square_algorithms);
}

fn demo_natural_is_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        if n.is_square() {
            println!("{n} is a perfect square");
        } else {
            println!("{n} is not a perfect square");
        }
    }
}

fn benchmark_natural_is_square_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_square()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.is_square())),
            ("rug", &mut |(n, _)| no_out!(n.is_perfect_square())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_is_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_square()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.is_square())),
            ("using checked_sqrt", &mut |n| {
                no_out!(n.checked_sqrt().is_some());
            }),
        ],
    );
}
