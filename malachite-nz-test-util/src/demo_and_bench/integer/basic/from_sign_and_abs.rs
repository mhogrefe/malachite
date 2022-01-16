use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz_test_util::generators::natural_bool_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_from_sign_and_abs);
    register_demo!(runner, demo_from_sign_and_abs_ref);

    register_bench!(runner, benchmark_from_sign_and_abs_evaluation_strategy);
}

fn demo_from_sign_and_abs(gm: GenMode, config: GenConfig, limit: usize) {
    for (abs, sign) in natural_bool_pair_gen().get(gm, &config).take(limit) {
        let abs_old = abs.clone();
        println!(
            "Integer::from_sign_and_abs({}, {}) = {}",
            sign,
            abs_old,
            Integer::from_sign_and_abs(sign, abs)
        );
    }
}

fn demo_from_sign_and_abs_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (abs, sign) in natural_bool_pair_gen().get(gm, &config).take(limit) {
        println!(
            "Integer::from_sign_and_abs_ref({}, {}) = {}",
            sign,
            abs,
            Integer::from_sign_and_abs_ref(sign, &abs)
        );
    }
}

fn benchmark_from_sign_and_abs_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_sign_and_abs(bool, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_bool_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("abs"),
        &mut [
            ("from_sign_and_abs", &mut |(abs, sign)| {
                no_out!(Integer::from_sign_and_abs(sign, abs))
            }),
            ("from_sign_and_abs_ref", &mut |(abs, sign)| {
                no_out!(Integer::from_sign_and_abs_ref(sign, &abs))
            }),
        ],
    );
}
