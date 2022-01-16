use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q_test_util::bench::bucketers::pair_2_rational_natural_max_bit_bucketer;
use malachite_q_test_util::generators::{rational_natural_pair_gen, rational_natural_pair_gen_rm};
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_partial_cmp_natural);
    register_demo!(runner, demo_natural_partial_cmp_rational);
    register_bench!(
        runner,
        benchmark_rational_partial_cmp_natural_library_comparison
    );
    register_bench!(
        runner,
        benchmark_natural_partial_cmp_rational_library_comparison
    );
}

fn demo_rational_partial_cmp_natural(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, &config).take(limit) {
        match x.partial_cmp(&y).unwrap() {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

fn demo_natural_partial_cmp_rational(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen().get(gm, &config).take(limit) {
        match y.partial_cmp(&x).unwrap() {
            Ordering::Less => println!("{} < {}", y, x),
            Ordering::Equal => println!("{} = {}", y, x),
            Ordering::Greater => println!("{} > {}", y, x),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_partial_cmp_natural_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.partial_cmp(&Natural)",
        BenchmarkType::LibraryComparison,
        rational_natural_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_partial_cmp_rational_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.partial_cmp(&Rational)",
        BenchmarkType::LibraryComparison,
        rational_natural_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y.partial_cmp(&x))),
            ("rug", &mut |((x, y), _)| no_out!(y.partial_cmp(&x))),
        ],
    );
}
