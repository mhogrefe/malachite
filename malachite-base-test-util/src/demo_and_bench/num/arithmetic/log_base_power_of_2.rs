use malachite_base::num::arithmetic::log_base_power_of_2::_ceiling_log_base_power_of_2_naive;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_21;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_log_base_power_of_2);
    register_unsigned_demos!(runner, demo_ceiling_log_base_power_of_2);
    register_unsigned_demos!(runner, demo_checked_log_base_power_of_2);
    register_unsigned_benches!(runner, benchmark_floor_log_base_power_of_2);
    register_unsigned_benches!(runner, benchmark_ceiling_log_base_power_of_2_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_log_base_power_of_2);
}

fn demo_floor_log_base_power_of_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, pow) in unsigned_pair_gen_var_21::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.floor_log_base_power_of_2({}) = {}",
            n,
            pow,
            n.floor_log_base_power_of_2(pow)
        );
    }
}

fn demo_ceiling_log_base_power_of_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, pow) in unsigned_pair_gen_var_21::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.ceiling_log_base_power_of_2({}) = {}",
            n,
            pow,
            n.ceiling_log_base_power_of_2(pow)
        );
    }
}

fn demo_checked_log_base_power_of_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, pow) in unsigned_pair_gen_var_21::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.checked_log_base_power_of_2({}) = {:?}",
            n,
            pow,
            n.checked_log_base_power_of_2(pow)
        );
    }
}

fn benchmark_floor_log_base_power_of_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_log_base_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_21::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.floor_log_base_power_of_2(pow))
        })],
    );
}

fn benchmark_ceiling_log_base_power_of_2_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_log_base_power_of_2(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_21::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, pow)| {
                no_out!(n.ceiling_log_base_power_of_2(pow))
            }),
            ("naive", &mut |(n, pow)| {
                no_out!(_ceiling_log_base_power_of_2_naive(n, pow))
            }),
        ],
    );
}

fn benchmark_checked_log_base_power_of_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_log_base_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_21::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.checked_log_base_power_of_2(pow))
        })],
    );
}
