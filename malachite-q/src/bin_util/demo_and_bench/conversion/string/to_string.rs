// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_3_rational_bit_bucketer;
use malachite_q::test_util::generators::{rational_gen, rational_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_to_string);
    register_demo!(runner, demo_rational_to_debug_string);

    register_bench!(runner, benchmark_rational_to_string_library_comparison);
    register_bench!(
        runner,
        benchmark_rational_to_debug_string_library_comparison
    );
}

fn demo_rational_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for q in rational_gen().get(gm, config).take(limit) {
        println!("{q}");
    }
}

fn demo_rational_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for q in rational_gen().get(gm, config).take(limit) {
        println!("{q:?}");
    }
}

fn benchmark_rational_to_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_string()",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_string())),
            ("num", &mut |(x, _, _)| no_out!(x.to_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_string())),
        ],
    );
}

fn benchmark_rational_to_debug_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_debug_string()",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_debug_string())),
            ("num", &mut |(x, _, _)| no_out!(x.to_debug_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_debug_string())),
        ],
    );
}
