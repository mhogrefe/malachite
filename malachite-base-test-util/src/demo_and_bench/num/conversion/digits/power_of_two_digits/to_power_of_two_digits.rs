use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{PowerOfTwoDigitIterable, PowerOfTwoDigits};
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_4;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_to_power_of_two_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_to_power_of_two_digits_desc);
    register_unsigned_unsigned_benches!(
        runner,
        benchmark_to_power_of_two_digits_asc_evaluation_strategy
    );
    register_unsigned_unsigned_benches!(
        runner,
        benchmark_to_power_of_two_digits_desc_evaluation_strategy
    );
}

fn demo_to_power_of_two_digits_asc<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, log_base) in unsigned_pair_gen_var_4::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_two_digits_asc({}) = {:?}",
            x,
            log_base,
            PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&x, log_base)
        );
    }
}

fn demo_to_power_of_two_digits_desc<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, log_base) in unsigned_pair_gen_var_4::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.to_power_of_two_digits_desc({}) = {:?}",
            x,
            log_base,
            PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&x, log_base)
        );
    }
}

fn benchmark_to_power_of_two_digits_asc_evaluation_strategy<
    T: PowerOfTwoDigits<U> + PowerOfTwoDigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "PowerOfTwoDigits::<{}>::to_power_of_two_digits_asc({}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_pair_gen_var_4::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(x, log_base)| {
                no_out!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(
                    &x, log_base
                ))
            }),
            (
                &format!("{}.power_of_two_digits(u64).collect_vec()", T::NAME),
                &mut |(x, log_base)| {
                    no_out!(
                        PowerOfTwoDigitIterable::<U>::power_of_two_digits(x, log_base)
                            .collect_vec()
                    )
                },
            ),
        ],
    );
}

fn benchmark_to_power_of_two_digits_desc_evaluation_strategy<
    T: PowerOfTwoDigits<U> + PowerOfTwoDigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "PowerOfTwoDigits::<{}>::to_power_of_two_digits_desc({}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_pair_gen_var_4::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(x, log_base)| {
                no_out!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(
                    &x, log_base
                ))
            }),
            (
                &format!("{}.power_of_two_digits(u64).rev().collect_vec()", T::NAME),
                &mut |(x, log_base)| {
                    no_out!(
                        PowerOfTwoDigitIterable::<U>::power_of_two_digits(x, log_base)
                            .rev()
                            .collect_vec()
                    )
                },
            ),
        ],
    );
}
