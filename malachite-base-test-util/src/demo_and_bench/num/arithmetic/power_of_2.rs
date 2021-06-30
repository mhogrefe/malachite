use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{unsigned_gen_var_15, unsigned_gen_var_16};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_power_of_2_unsigned);
    register_signed_demos!(runner, demo_power_of_2_signed);
    register_unsigned_benches!(runner, benchmark_power_of_2_unsigned);
    register_signed_benches!(runner, benchmark_power_of_2_signed);
}

fn demo_power_of_2_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for pow in unsigned_gen_var_15::<T>().get(gm, &config).take(limit) {
        println!("2^{} = {}", pow, T::power_of_2(pow));
    }
}

fn demo_power_of_2_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for pow in unsigned_gen_var_16::<T>().get(gm, &config).take(limit) {
        println!("2^{} = {}", pow, T::power_of_2(pow));
    }
}

fn benchmark_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.power_of_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_15::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |pow| no_out!(T::power_of_2(pow)))],
    );
}

fn benchmark_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.power_of_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_16::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |pow| no_out!(T::power_of_2(pow)))],
    );
}
