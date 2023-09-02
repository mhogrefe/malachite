use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::arithmetic::sign::num_sign;
use malachite_q::test_util::bench::bucketers::triple_3_rational_bit_bucketer;
use malachite_q::test_util::generators::{rational_gen, rational_gen_nrm};
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_sign);
    register_bench!(runner, benchmark_integer_sign_library_comparison);
}

fn demo_integer_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        match x.sign() {
            Ordering::Less => println!("{x} is negative"),
            Ordering::Equal => println!("{x} is zero"),
            Ordering::Greater => println!("{x} is positive"),
        }
    }
}

fn benchmark_integer_sign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sign()",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.sign())),
            ("num", &mut |(x, _, _)| no_out!(num_sign(&x))),
            ("rug", &mut |(_, x, _)| no_out!(x.cmp0())),
        ],
    );
}
