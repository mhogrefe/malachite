// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    integer_natural_max_bit_bucketer, pair_2_integer_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_natural_pair_gen, integer_natural_pair_gen_rm};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_partial_cmp_abs_natural);
    register_demo!(runner, demo_natural_partial_cmp_abs_integer);
    register_demo!(runner, demo_integer_lt_abs_natural);
    register_demo!(runner, demo_integer_gt_abs_natural);
    register_demo!(runner, demo_integer_le_abs_natural);
    register_demo!(runner, demo_integer_ge_abs_natural);
    register_demo!(runner, demo_natural_lt_abs_integer);
    register_demo!(runner, demo_natural_gt_abs_integer);
    register_demo!(runner, demo_natural_le_abs_integer);
    register_demo!(runner, demo_natural_ge_abs_integer);

    register_bench!(
        runner,
        benchmark_integer_partial_cmp_abs_natural_library_comparison
    );
    register_bench!(
        runner,
        benchmark_natural_partial_cmp_abs_integer_library_comparison
    );
    register_bench!(runner, benchmark_integer_lt_abs_natural);
    register_bench!(runner, benchmark_integer_gt_abs_natural);
    register_bench!(runner, benchmark_integer_le_abs_natural);
    register_bench!(runner, benchmark_integer_ge_abs_natural);
    register_bench!(runner, benchmark_natural_lt_abs_integer);
    register_bench!(runner, benchmark_natural_gt_abs_integer);
    register_bench!(runner, benchmark_natural_le_abs_integer);
    register_bench!(runner, benchmark_natural_ge_abs_integer);
}

fn demo_integer_partial_cmp_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp_abs(&y).unwrap() {
            Less => println!("|{x}| < |{y}|"),
            Equal => println!("|{x}| = |{y}|"),
            Greater => println!("|{x}| > |{y}|"),
        }
    }
}

fn demo_natural_partial_cmp_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        match y.partial_cmp_abs(&x).unwrap() {
            Less => println!("|{y}| < |{x}|"),
            Equal => println!("|{y}| = |{x}|"),
            Greater => println!("|{y}| > |{x}|"),
        }
    }
}

fn demo_integer_lt_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if x.lt_abs(&y) {
            println!("|{x}| < |{y}|");
        } else {
            println!("|{x}| ≮ |{y}|");
        }
    }
}

fn demo_integer_gt_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if x.gt_abs(&y) {
            println!("|{x}| > |{y}|");
        } else {
            println!("|{x}| ≯ |{y}|");
        }
    }
}

fn demo_integer_le_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if x.le_abs(&y) {
            println!("|{x}| ≤ |{y}|");
        } else {
            println!("|{x}| ≰ |{y}|");
        }
    }
}

fn demo_integer_ge_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if x.ge_abs(&y) {
            println!("|{x}| ≥ |{y}|");
        } else {
            println!("|{x}| ≱ |{y}|");
        }
    }
}

fn demo_natural_lt_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if y.lt_abs(&x) {
            println!("|{y}| < |{x}|");
        } else {
            println!("|{y}| ≮ |{x}|");
        }
    }
}

fn demo_natural_gt_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if y.gt_abs(&x) {
            println!("|{y}| > |{x}|");
        } else {
            println!("|{y}| ≯ |{x}|");
        }
    }
}

fn demo_natural_le_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if y.le_abs(&x) {
            println!("|{y}| ≤ |{x}|");
        } else {
            println!("|{y}| ≰ |{x}|");
        }
    }
}

fn demo_natural_ge_abs_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_natural_pair_gen().get(gm, config).take(limit) {
        if y.ge_abs(&x) {
            println!("|{y}| ≥ |{x}|");
        } else {
            println!("|{y}| ≱ |{x}|");
        }
    }
}

fn benchmark_integer_partial_cmp_abs_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.partial_cmp_abs(&Natural)",
        BenchmarkType::LibraryComparison,
        integer_natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.partial_cmp_abs(&y))
            }),
            ("rug", &mut |((x, y), _)| no_out!(x.cmp_abs(&y))),
        ],
    );
}

fn benchmark_natural_partial_cmp_abs_integer_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.partial_cmp_abs(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(y.partial_cmp_abs(&x))
            }),
            ("rug", &mut |((x, y), _)| no_out!(y.cmp_abs(&x))),
        ],
    );
}

fn benchmark_integer_lt_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.lt_abs(&Natural)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_integer_gt_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.gt_abs(&Natural)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_integer_le_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.le_abs(&Natural)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_integer_ge_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ge_abs(&Natural)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}

fn benchmark_natural_lt_abs_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lt_abs(&Integer)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.lt_abs(&x)))],
    );
}

fn benchmark_natural_gt_abs_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gt_abs(&Integer)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.gt_abs(&x)))],
    );
}

fn benchmark_natural_le_abs_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.le_abs(&Integer)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.le_abs(&x)))],
    );
}

fn benchmark_natural_ge_abs_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ge_abs(&Integer)",
        BenchmarkType::Single,
        integer_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.ge_abs(&x)))],
    );
}
