use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_is_power_of_2);
    register_bench!(runner, benchmark_rational_is_power_of_2);
}

fn demo_rational_is_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        if x.is_power_of_2() {
            println!("{x} is a power of 2");
        } else {
            println!("{x} is not a power of 2");
        }
    }
}

fn benchmark_rational_is_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.is_power_of_2()",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_power_of_2()))],
    );
}
