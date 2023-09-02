use malachite_base::num::conversion::traits::IsInteger;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_is_integer);
    register_bench!(runner, benchmark_rational_is_integer);
}

fn demo_rational_is_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        if n.is_integer() {
            println!("{n} is an integer");
        } else {
            println!("{n} is not an integer");
        }
    }
}

fn benchmark_rational_is_integer(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.is_integer()",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_integer()))],
    );
}
