use crate::bench::bucketers::integer_bit_bucketer;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_even);
    register_demo!(runner, demo_integer_odd);

    register_bench!(runner, benchmark_integer_even);
    register_bench!(runner, benchmark_integer_odd);
}

fn demo_integer_even(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        if n.even() {
            println!("{} is even", n);
        } else {
            println!("{} is not even", n);
        }
    }
}

fn demo_integer_odd(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        if n.even() {
            println!("{} is odd", n);
        } else {
            println!("{} is not odd", n);
        }
    }
}

fn benchmark_integer_even(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.even()",
        BenchmarkType::Single,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.even()))],
    );
}

fn benchmark_integer_odd(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.odd()",
        BenchmarkType::Single,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.odd()))],
    );
}
