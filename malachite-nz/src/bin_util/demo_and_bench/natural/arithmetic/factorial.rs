use malachite_base::num::arithmetic::traits::{
    DoubleFactorial, Factorial, Multifactorial, Subfactorial,
};
use malachite_base::test_util::bench::bucketers::{
    unsigned_direct_bucketer, usize_convertible_pair_max_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_pair_gen_var_18};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::factorial::{
    double_factorial_naive, factorial_naive, multifactorial_naive,
};
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_factorial);
    register_demo!(runner, demo_double_factorial);
    register_demo!(runner, demo_multifactorial);
    register_demo!(runner, demo_subfactorial);

    register_bench!(runner, benchmark_factorial_algorithms);
    register_bench!(runner, benchmark_double_factorial_algorithms);
    register_bench!(runner, benchmark_multifactorial_algorithms);
    register_bench!(runner, benchmark_subfactorial);
}

fn demo_factorial(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, &config).take(limit) {
        println!("{}! = {}", n, Natural::factorial(n));
    }
}

fn demo_double_factorial(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, &config).take(limit) {
        println!("{}!! = {}", n, Natural::double_factorial(n));
    }
}

fn demo_multifactorial(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in unsigned_pair_gen_var_18().get(gm, &config).take(limit) {
        if m <= 5 {
            print!("{}", n);
            for _ in 0..m {
                print!("!");
            }
            println!(" = {}", Natural::multifactorial(n, m));
        } else {
            println!("{}[!^{}] = {}", n, m, Natural::multifactorial(n, m));
        }
    }
}

fn demo_subfactorial(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, &config).take(limit) {
        println!("!{} = {}", n, Natural::subfactorial(n));
    }
}

fn benchmark_factorial_algorithms(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.factorial(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::factorial(n))),
            ("naive", &mut |n| no_out!(factorial_naive(n))),
        ],
    );
}

fn benchmark_double_factorial_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.double_factorial(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::double_factorial(n))),
            ("naive", &mut |n| no_out!(double_factorial_naive(n))),
        ],
    );
}

fn benchmark_multifactorial_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.multifactorial(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_18().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "m"),
        &mut [
            ("default", &mut |(n, m)| {
                no_out!(Natural::multifactorial(n, m))
            }),
            ("naive", &mut |(n, m)| no_out!(multifactorial_naive(n, m))),
        ],
    );
}

fn benchmark_subfactorial(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.subfactorial(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(Natural::subfactorial(n)))],
    );
}
