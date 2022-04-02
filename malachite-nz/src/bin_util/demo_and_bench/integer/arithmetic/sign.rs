use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_gen, integer_gen_nrm};
use malachite_nz::test_util::integer::arithmetic::sign::num_sign;
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_sign);
    register_bench!(runner, benchmark_integer_sign_library_comparison);
}

fn demo_integer_sign(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

fn benchmark_integer_sign_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sign()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(n.sign())),
            ("num", &mut |(n, _, _)| no_out!(num_sign(&n))),
            ("rug", &mut |(_, n, _)| no_out!(n.cmp0())),
        ],
    );
}
