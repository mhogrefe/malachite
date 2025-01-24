// Copyright © 2025 Mikhail Hogrefe
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
use malachite_q::test_util::bench::bucketers::rational_natural_max_bit_bucketer;
use malachite_q::test_util::generators::rational_natural_pair_gen;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_partial_cmp_abs_natural);
    register_demo!(runner, demo_natural_partial_cmp_abs_rational);
    register_demo!(runner, demo_rational_lt_abs_natural);
    register_demo!(runner, demo_rational_gt_abs_natural);
    register_demo!(runner, demo_rational_le_abs_natural);
    register_demo!(runner, demo_rational_ge_abs_natural);
    register_demo!(runner, demo_natural_lt_abs_rational);
    register_demo!(runner, demo_natural_gt_abs_rational);
    register_demo!(runner, demo_natural_le_abs_rational);
    register_demo!(runner, demo_natural_ge_abs_rational);

    register_bench!(runner, benchmark_rational_partial_cmp_abs_natural);
    register_bench!(runner, benchmark_natural_partial_cmp_abs_rational);
    register_bench!(runner, benchmark_rational_lt_abs_natural);
    register_bench!(runner, benchmark_rational_gt_abs_natural);
    register_bench!(runner, benchmark_rational_le_abs_natural);
    register_bench!(runner, benchmark_rational_ge_abs_natural);
    register_bench!(runner, benchmark_natural_lt_abs_rational);
    register_bench!(runner, benchmark_natural_gt_abs_rational);
    register_bench!(runner, benchmark_natural_le_abs_rational);
    register_bench!(runner, benchmark_natural_ge_abs_rational);
}

fn demo_rational_partial_cmp_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp_abs(&y).unwrap() {
            Less => println!("|{x}| < |{y}|"),
            Equal => println!("|{x}| = |{y}|"),
            Greater => println!("|{x}| > |{y}|"),
        }
    }
}

fn demo_natural_partial_cmp_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        match y.partial_cmp_abs(&x).unwrap() {
            Less => println!("|{y}| < |{x}|"),
            Equal => println!("|{y}| = |{x}|"),
            Greater => println!("|{y}| > |{x}|"),
        }
    }
}

fn demo_rational_lt_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if x.lt_abs(&y) {
            println!("|{x}| < |{y}|");
        } else {
            println!("|{x}| ≮ |{y}|");
        }
    }
}

fn demo_rational_gt_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if x.gt_abs(&y) {
            println!("|{x}| > |{y}|");
        } else {
            println!("|{x}| ≯ |{y}|");
        }
    }
}

fn demo_rational_le_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if x.le_abs(&y) {
            println!("|{x}| ≤ |{y}|");
        } else {
            println!("|{x}| ≰ |{y}|");
        }
    }
}

fn demo_rational_ge_abs_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if x.ge_abs(&y) {
            println!("|{x}| ≥ |{y}|");
        } else {
            println!("|{x}| ≱ |{y}|");
        }
    }
}

fn demo_natural_lt_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if y.lt_abs(&x) {
            println!("|{y}| < |{x}|");
        } else {
            println!("|{y}| ≮ |{x}|");
        }
    }
}

fn demo_natural_gt_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if y.gt_abs(&x) {
            println!("|{y}| > |{x}|");
        } else {
            println!("|{y}| ≯ |{x}|");
        }
    }
}

fn demo_natural_le_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if y.le_abs(&x) {
            println!("|{y}| ≤ |{x}|");
        } else {
            println!("|{y}| ≰ |{x}|");
        }
    }
}

fn demo_natural_ge_abs_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, config).take(limit) {
        if y.ge_abs(&x) {
            println!("|{y}| ≥ |{x}|");
        } else {
            println!("|{y}| ≱ |{x}|");
        }
    }
}

fn benchmark_rational_partial_cmp_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.partial_cmp_abs(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_natural_partial_cmp_abs_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.partial_cmp_abs(&Rational)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}

fn benchmark_rational_lt_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.lt_abs(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_rational_gt_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.gt_abs(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_rational_le_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.le_abs(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_rational_ge_abs_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ge_abs(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}

fn benchmark_natural_lt_abs_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lt_abs(&Rational)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.lt_abs(&x)))],
    );
}

fn benchmark_natural_gt_abs_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gt_abs(&Rational)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.gt_abs(&x)))],
    );
}

fn benchmark_natural_le_abs_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.le_abs(&Rational)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.le_abs(&x)))],
    );
}

fn benchmark_natural_ge_abs_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ge_abs(&Rational)",
        BenchmarkType::Single,
        rational_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.ge_abs(&x)))],
    );
}
