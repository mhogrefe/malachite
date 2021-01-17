use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOfTwoDigits;

use malachite_base_test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_3,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_from_power_of_two_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_from_power_of_two_digits_desc);
    register_unsigned_unsigned_benches!(runner, benchmark_from_power_of_two_digits_asc);
    register_unsigned_unsigned_benches!(runner, benchmark_from_power_of_two_digits_desc);
}

fn demo_from_power_of_two_digits_asc<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (xs, log_base) in unsigned_vec_unsigned_pair_gen_var_2::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::from_power_of_two_digits_asc({}, {:?}) = {}",
            T::NAME,
            log_base,
            xs,
            T::from_power_of_two_digits_asc(log_base, xs.iter().cloned())
        );
    }
}

fn demo_from_power_of_two_digits_desc<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (xs, log_base) in unsigned_vec_unsigned_pair_gen_var_3::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::from_power_of_two_digits_desc({}, {:?}) = {}",
            T::NAME,
            log_base,
            xs,
            T::from_power_of_two_digits_desc(log_base, xs.iter().cloned())
        );
    }
}

fn benchmark_from_power_of_two_digits_asc<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_power_of_two_digits_asc<I: Iterator<Item={}>>(u64, I)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_2::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, log_base)| {
            no_out!(T::from_power_of_two_digits_asc(log_base, xs.into_iter()))
        })],
    );
}

fn benchmark_from_power_of_two_digits_desc<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_power_of_two_digits_asc<I: Iterator<Item={}>>(u64, I)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_3::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, log_base)| {
            no_out!(T::from_power_of_two_digits_desc(log_base, xs.into_iter()))
        })],
    );
}
