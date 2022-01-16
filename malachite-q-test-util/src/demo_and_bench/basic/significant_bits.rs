use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q_test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q_test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_significant_bits);

    register_bench!(runner, benchmark_significant_bits);
}

fn demo_significant_bits(gm: GenMode, config: GenConfig, limit: usize) {
    for x in rational_gen().get(gm, &config).take(limit) {
        println!("significant_bits({}) = {}", x, x.significant_bits());
    }
}

fn benchmark_significant_bits(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.significant_bits()",
        BenchmarkType::Single,
        rational_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.significant_bits()))],
    );
}
