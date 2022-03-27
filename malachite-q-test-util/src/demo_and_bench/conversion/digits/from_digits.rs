use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::bench::bucketers::rational_from_digits_bucketer;
use malachite_nz_test_util::generators::{large_type_gen_var_25, large_type_gen_var_26};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_digits);
    register_demo!(runner, demo_rational_from_digits_ref);
    register_demo!(runner, demo_rational_from_digits_decimal);
    register_bench!(runner, benchmark_rational_from_digits_evaluation_strategy);
}

fn demo_rational_from_digits(gm: GenMode, config: GenConfig, limit: usize) {
    for (base, before_point, after_point) in large_type_gen_var_25().get(gm, &config).take(limit) {
        println!(
            "from_digits({}, {:?}, {}) = {}",
            base,
            before_point.clone(),
            after_point.clone(),
            Rational::from_digits(&base, before_point, after_point)
        );
    }
}

fn demo_rational_from_digits_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (base, before_point, after_point) in large_type_gen_var_25().get(gm, &config).take(limit) {
        println!(
            "from_digits_ref({}, {:?}, {:?}) = {}",
            base,
            before_point,
            after_point,
            Rational::from_digits_ref(&base, &before_point, &after_point)
        );
    }
}

fn demo_rational_from_digits_decimal(gm: GenMode, config: GenConfig, limit: usize) {
    for (before_point, after_point) in large_type_gen_var_26().get(gm, &config).take(limit) {
        println!(
            "from_digits(1, {:?}, {}) = {}",
            before_point.clone(),
            after_point.clone(),
            Rational::from_digits(&Natural::from(10u32), before_point, after_point)
        );
    }
}

fn benchmark_rational_from_digits_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_digits(Natural, &[Natural], &RationalSequence<Natural>)",
        BenchmarkType::EvaluationStrategy,
        large_type_gen_var_25().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_from_digits_bucketer(),
        &mut [
            (
                "Rational::from_digits(base, Vec<Natural>, RationalSequence<Natural>)",
                &mut |(base, before_point, after_point)| {
                    no_out!(Rational::from_digits(&base, before_point, after_point))
                },
            ),
            (
                "Rational::from_digits_ref(base, &[Natural], &RationalSequence<Natural>)",
                &mut |(base, before_point, after_point)| {
                    no_out!(Rational::from_digits_ref(
                        &base,
                        &before_point,
                        &after_point
                    ))
                },
            ),
        ],
    );
}
