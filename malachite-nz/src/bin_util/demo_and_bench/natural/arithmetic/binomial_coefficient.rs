use malachite_base::num::arithmetic::traits::BinomialCoefficient;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_pair_gen_var_15, natural_pair_gen_var_15_rm};
use malachite_nz::test_util::natural::arithmetic::binomial_coefficient::binomial_coefficient_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_binomial_coefficient);
    register_demo!(runner, demo_natural_binomial_coefficient_ref);

    register_bench!(
        runner,
        benchmark_natural_binomial_coefficient_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_binomial_coefficient_algorithms);
    register_bench!(
        runner,
        benchmark_natural_binomial_coefficient_library_comparison
    );
}

fn demo_natural_binomial_coefficient(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, k) in natural_pair_gen_var_15().get(gm, &config).take(limit) {
        let n_orig = n.clone();
        let k_orig = k.clone();
        println!(
            "C({}, {}) = {}",
            n_orig,
            k_orig,
            Natural::binomial_coefficient(n, k)
        );
    }
}

fn demo_natural_binomial_coefficient_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, k) in natural_pair_gen_var_15().get(gm, &config).take(limit) {
        println!(
            "C({}, {}) = {}",
            n,
            k,
            Natural::binomial_coefficient(&n, &k)
        );
    }
}

fn benchmark_natural_binomial_coefficient_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.binomial_coefficient(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_15().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.binomial_coefficient(Natural, Natural)",
                &mut |(n, k)| no_out!(Natural::binomial_coefficient(n, k)),
            ),
            (
                "Natural.binomial_coefficient(&Natural, &Natural)",
                &mut |(n, k)| no_out!(Natural::binomial_coefficient(&n, &k)),
            ),
        ],
    );
}

fn benchmark_natural_binomial_coefficient_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.binomial_coefficient(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_15().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(n, k)| {
                no_out!(Natural::binomial_coefficient(n, k))
            }),
            ("naive", &mut |(n, k)| {
                no_out!(binomial_coefficient_naive(n, k))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_binomial_coefficient_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.binomial_coefficient(Natural, Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_15_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (n, k))| {
                no_out!(Natural::binomial_coefficient(n, k))
            }),
            ("rug", &mut |((n, k), _)| no_out!(n.binomial(k))),
        ],
    );
}
