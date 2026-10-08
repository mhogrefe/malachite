// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_to_string);
    register_demo!(runner, demo_rational_vector_to_debug_string);
    register_bench!(runner, benchmark_rational_vector_to_string);
    register_bench!(runner, benchmark_rational_vector_to_debug_string);
    register_bench!(
        runner,
        benchmark_rational_vector_to_string_by_language_algorithms
    );
}

fn demo_rational_vector_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("{v}");
    }
}

fn demo_rational_vector_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("{v:?}");
    }
}

fn benchmark_rational_vector_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.to_string()",
        BenchmarkType::Single,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.to_string()))],
    );
}

fn benchmark_rational_vector_to_debug_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.to_debug_string()",
        BenchmarkType::Single,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.to_debug_string()))],
    );
}

fn benchmark_rational_vector_to_string_by_language_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.to_string()",
        BenchmarkType::Algorithms,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("plain", &mut |v| no_out!(v.to_string())),
            ("LaTeX", &mut |v| no_out!(v.to_latex_string())),
            ("Typst", &mut |v| no_out!(v.to_typst_string())),
        ],
    );
}
