// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_integer_max_bit_bucketer, pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen, integer_pair_gen_rm};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_cmp_abs);
    register_demo!(runner, demo_integer_lt_abs);
    register_demo!(runner, demo_integer_gt_abs);
    register_demo!(runner, demo_integer_le_abs);
    register_demo!(runner, demo_integer_ge_abs);

    register_bench!(runner, benchmark_integer_cmp_abs_library_comparison);
    register_bench!(runner, benchmark_integer_lt_abs);
    register_bench!(runner, benchmark_integer_gt_abs);
    register_bench!(runner, benchmark_integer_le_abs);
    register_bench!(runner, benchmark_integer_ge_abs);
}

fn demo_integer_cmp_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        match x.cmp_abs(&y) {
            Less => println!("|{x}| < |{y}|"),
            Equal => println!("|{x}| = |{y}|"),
            Greater => println!("|{x}| > |{y}|"),
        }
    }
}

fn demo_integer_lt_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        if x.lt_abs(&y) {
            println!("|{x}| < |{y}|");
        } else {
            println!("|{x}| ≮ |{y}|");
        }
    }
}

fn demo_integer_gt_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        if x.gt_abs(&y) {
            println!("|{x}| > |{y}|");
        } else {
            println!("|{x}| ≯ |{y}|");
        }
    }
}

fn demo_integer_le_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        if x.le_abs(&y) {
            println!("|{x}| ≤ |{y}|");
        } else {
            println!("|{x}| ≰ |{y}|");
        }
    }
}

fn demo_integer_ge_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        if x.ge_abs(&y) {
            println!("|{x}| ≥ |{y}|");
        } else {
            println!("|{x}| ≱ |{y}|");
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_integer_cmp_abs_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.cmp_abs(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.cmp_abs(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.cmp_abs(&y))),
        ],
    );
}

fn benchmark_integer_lt_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.lt_abs(&Integer)",
        BenchmarkType::Single,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_integer_gt_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.gt_abs(&Integer)",
        BenchmarkType::Single,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_integer_le_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.le_abs(&Integer)",
        BenchmarkType::Single,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_integer_ge_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.ge_abs(&Integer)",
        BenchmarkType::Single,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}
