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
use malachite_base::test_util::bench::bucketers::vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_to_string);
    register_demo!(runner, demo_unsigned_vector_to_debug_string);
    register_bench!(runner, benchmark_unsigned_vector_to_string);
    register_bench!(runner, benchmark_unsigned_vector_to_debug_string);
    register_bench!(
        runner,
        benchmark_unsigned_vector_to_string_by_language_algorithms
    );
}

fn demo_unsigned_vector_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("{v}");
    }
}

fn demo_unsigned_vector_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("{v:?}");
    }
}

fn benchmark_unsigned_vector_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Vector<u64>.to_string()",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.to_string()))],
    );
}

fn benchmark_unsigned_vector_to_debug_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Vector<u64>.to_debug_string()",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| no_out!(v.to_debug_string()))],
    );
}

fn benchmark_unsigned_vector_to_string_by_language_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Vector<u64>.to_string()",
        BenchmarkType::Algorithms,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vector_dimension_bucketer("v"),
        &mut [
            ("plain", &mut |v| no_out!(v.to_string())),
            ("LaTeX", &mut |v| no_out!(v.to_latex_string())),
            ("Typst", &mut |v| no_out!(v.to_typst_string())),
        ],
    );
}
