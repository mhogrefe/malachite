use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q_test_util::bench::bucketers::pair_2_rational_integer_max_bit_bucketer;
use malachite_q_test_util::generators::{rational_integer_pair_gen, rational_integer_pair_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_partial_eq_integer);
    register_demo!(runner, demo_integer_partial_eq_rational);
    register_bench!(
        runner,
        benchmark_rational_partial_eq_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_integer_partial_eq_rational_library_comparison
    );
}

fn demo_rational_partial_eq_integer(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in rational_integer_pair_gen().get(gm, &config).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

fn demo_integer_partial_eq_rational(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in rational_integer_pair_gen().get(gm, &config).take(limit) {
        if y == x {
            println!("{} = {}", y, x);
        } else {
            println!("{} ≠ {}", y, x);
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_partial_eq_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational == Integer",
        BenchmarkType::LibraryComparison,
        rational_integer_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_partial_eq_rational_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer == Rational",
        BenchmarkType::LibraryComparison,
        rational_integer_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y == x)),
            ("rug", &mut |((x, y), _)| no_out!(y == x)),
        ],
    );
}
