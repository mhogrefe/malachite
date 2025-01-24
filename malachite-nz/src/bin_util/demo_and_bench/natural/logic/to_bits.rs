// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::num::logic::bit_convertible::{to_bits_asc_alt, to_bits_desc_alt};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::logic::to_bits::{to_bits_asc_naive, to_bits_desc_naive};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_to_bits_asc);
    register_demo!(runner, demo_natural_to_bits_desc);

    register_bench!(runner, benchmark_natural_to_bits_asc_evaluation_strategy);
    register_bench!(runner, benchmark_natural_to_bits_asc_algorithms);
    register_bench!(runner, benchmark_natural_to_bits_desc_evaluation_strategy);
    register_bench!(runner, benchmark_natural_to_bits_desc_algorithms);
}

fn demo_natural_to_bits_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("to_bits_asc({}) = {:?}", n, n.to_bits_asc());
    }
}

fn demo_natural_to_bits_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("to_bits_desc({}) = {:?}", n, n.to_bits_desc());
    }
}

fn benchmark_natural_to_bits_asc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_bits_asc()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Natural.to_bits_asc()", &mut |n| no_out!(n.to_bits_asc())),
            ("Natural.bits().collect_vec()", &mut |n| {
                no_out!(n.bits().collect_vec())
            }),
        ],
    );
}

fn benchmark_natural_to_bits_asc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_bits_asc()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.to_bits_asc())),
            ("alt", &mut |n| no_out!(to_bits_asc_alt(&n))),
            ("naive", &mut |n| no_out!(to_bits_asc_naive(&n))),
        ],
    );
}

fn benchmark_natural_to_bits_desc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_bits_desc()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Natural.to_bits_asc()", &mut |n| no_out!(n.to_bits_desc())),
            ("Natural.bits().rev().collect_vec()", &mut |n| {
                no_out!(n.bits().rev().collect_vec())
            }),
        ],
    );
}

fn benchmark_natural_to_bits_desc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_bits_desc()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| no_out!(n.to_bits_desc())),
            ("alt", &mut |n| no_out!(to_bits_desc_alt(&n))),
            ("naive", &mut |n| no_out!(to_bits_desc_naive(&n))),
        ],
    );
}
