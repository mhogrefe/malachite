use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::bench::bucketers::{
    pair_integer_max_bit_bucketer, pair_natural_max_bit_bucketer,
    triple_1_2_natural_max_bit_bucketer, triple_3_pair_integer_max_bit_bucketer,
};
use malachite_nz_test_util::generators::{
    integer_pair_gen_var_1, integer_pair_gen_var_1_nrm, natural_natural_bool_triple_gen_var_1,
    natural_pair_gen_var_5,
};
use malachite_q::Rational;
use num::BigRational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_from_naturals);
    register_demo!(runner, demo_from_naturals_ref);
    register_demo!(runner, demo_from_integers);
    register_demo!(runner, demo_from_integers_ref);
    register_demo!(runner, demo_from_sign_and_naturals);
    register_demo!(runner, demo_from_sign_and_naturals_ref);

    register_bench!(runner, benchmark_from_naturals_evaluation_strategy);
    register_bench!(runner, benchmark_from_integers_evaluation_strategy);
    register_bench!(runner, benchmark_from_integers_library_comparison);
    register_bench!(runner, benchmark_from_sign_and_naturals_evaluation_strategy);
}

fn demo_from_naturals(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, d) in natural_pair_gen_var_5().get(gm, &config).take(limit) {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "Rational::from_naturals({}, {}) = {}",
            n_old,
            d_old,
            Rational::from_naturals(n, d)
        );
    }
}

fn demo_from_naturals_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, d) in natural_pair_gen_var_5().get(gm, &config).take(limit) {
        println!(
            "Rational::from_naturals_ref({}, {}) = {}",
            n,
            d,
            Rational::from_naturals_ref(&n, &d)
        );
    }
}

fn demo_from_integers(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, d) in integer_pair_gen_var_1().get(gm, &config).take(limit) {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "Rational::from_integers({}, {}) = {}",
            n_old,
            d_old,
            Rational::from_integers(n, d)
        );
    }
}

fn demo_from_integers_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, d) in integer_pair_gen_var_1().get(gm, &config).take(limit) {
        println!(
            "Rational::from_naturals_ref({}, {}) = {}",
            n,
            d,
            Rational::from_integers_ref(&n, &d)
        );
    }
}

fn demo_from_sign_and_naturals(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, d, sign) in natural_natural_bool_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "Rational::from_sign_and_naturals({}, {}, {}) = {}",
            sign,
            n_old,
            d_old,
            Rational::from_sign_and_naturals(sign, n, d)
        );
    }
}

fn demo_from_sign_and_naturals_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, d, sign) in natural_natural_bool_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Rational::from_sign_and_naturals_ref({}, {}, {}) = {}",
            sign,
            n,
            d,
            Rational::from_sign_and_naturals_ref(sign, &n, &d)
        );
    }
}

fn benchmark_from_naturals_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_naturals(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("n", "d"),
        &mut [
            ("from_naturals", &mut |(n, d)| {
                no_out!(Rational::from_naturals(n, d))
            }),
            ("from_naturals_ref", &mut |(n, d)| {
                no_out!(Rational::from_naturals_ref(&n, &d))
            }),
        ],
    );
}

fn benchmark_from_integers_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_integers(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("n", "d"),
        &mut [
            ("from_integers", &mut |(n, d)| {
                no_out!(Rational::from_integers(n, d))
            }),
            ("from_integers_ref", &mut |(n, d)| {
                no_out!(Rational::from_integers_ref(&n, &d))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_from_integers_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_integers(Integer, Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_integer_max_bit_bucketer("n", "d"),
        &mut [
            ("Malachite", &mut |(_, _, (n, d))| {
                no_out!(Rational::from_integers(n, d))
            }),
            ("num", &mut |((n, d), _, _)| no_out!(BigRational::new(n, d))),
            ("rug", &mut |(_, (n, d), _)| {
                no_out!(rug::Rational::from((n, d)))
            }),
        ],
    );
}

fn benchmark_from_sign_and_naturals_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_sign_and_naturals(bool, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_bool_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_max_bit_bucketer("n", "d"),
        &mut [
            ("from_sign_and_naturals", &mut |(n, d, sign)| {
                no_out!(Rational::from_sign_and_naturals(sign, n, d))
            }),
            ("from_sign_and_naturals_ref", &mut |(n, d, sign)| {
                no_out!(Rational::from_sign_and_naturals_ref(sign, &n, &d))
            }),
        ],
    );
}
