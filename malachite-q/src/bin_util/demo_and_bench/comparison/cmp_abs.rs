use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_2_pair_rational_max_bit_bucketer, pair_rational_max_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_pair_gen, rational_pair_gen_rm};
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_cmp_abs);
    register_demo!(runner, demo_rational_lt_abs);
    register_demo!(runner, demo_rational_gt_abs);
    register_demo!(runner, demo_rational_le_abs);
    register_demo!(runner, demo_rational_ge_abs);

    register_bench!(runner, benchmark_rational_cmp_abs_library_comparison);
    register_bench!(runner, benchmark_rational_lt_abs);
    register_bench!(runner, benchmark_rational_gt_abs);
    register_bench!(runner, benchmark_rational_le_abs);
    register_bench!(runner, benchmark_rational_ge_abs);
}

fn demo_rational_cmp_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        match x.cmp_abs(&y) {
            Ordering::Less => println!("|{x}| < |{y}|"),
            Ordering::Equal => println!("|{x}| = |{y}|"),
            Ordering::Greater => println!("|{x}| > |{y}|"),
        }
    }
}

fn demo_rational_lt_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        if x.lt_abs(&y) {
            println!("|{x}| < |{y}|");
        } else {
            println!("|{x}| ≮ |{y}|");
        }
    }
}

fn demo_rational_gt_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        if x.gt_abs(&y) {
            println!("|{x}| > |{y}|");
        } else {
            println!("|{x}| ≯ |{y}|");
        }
    }
}

fn demo_rational_le_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        if x.le_abs(&y) {
            println!("|{x}| ≤ |{y}|");
        } else {
            println!("|{x}| ≰ |{y}|");
        }
    }
}

fn demo_rational_ge_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        if x.ge_abs(&y) {
            println!("|{x}| ≥ |{y}|");
        } else {
            println!("|{x}| ≱ |{y}|");
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_cmp_abs_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.cmp_abs(&Rational)",
        BenchmarkType::LibraryComparison,
        rational_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.cmp_abs(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.cmp_abs(&y))),
        ],
    );
}

fn benchmark_rational_lt_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.lt_abs(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_rational_gt_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.gt_abs(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_rational_le_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.le_abs(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_rational_ge_abs(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.ge_abs(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}
