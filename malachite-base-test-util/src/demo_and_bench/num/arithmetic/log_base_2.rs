use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::primitive_int_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_gen_var_1;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_log_base_2);
    register_unsigned_demos!(runner, demo_ceiling_log_base_2);
    register_unsigned_demos!(runner, demo_checked_log_base_2);
    register_unsigned_benches!(runner, benchmark_floor_log_base_2);
    register_unsigned_benches!(runner, benchmark_ceiling_log_base_2);
    register_unsigned_benches!(runner, benchmark_checked_log_base_2);
}

fn demo_floor_log_base_2<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_1::<T>().get(gm, &config).take(limit) {
        println!("{}.floor_log_base_2() = {}", n, n.floor_log_base_2());
    }
}

fn demo_ceiling_log_base_2<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_1::<T>().get(gm, &config).take(limit) {
        println!("{}.ceiling_log_base_2() = {}", n, n.ceiling_log_base_2());
    }
}

fn demo_checked_log_base_2<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_1::<T>().get(gm, &config).take(limit) {
        println!("{}.checked_log_base_2() = {:?}", n, n.checked_log_base_2());
    }
}

fn benchmark_floor_log_base_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_log_base_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2()))],
    );
}

fn benchmark_ceiling_log_base_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_log_base_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2()))],
    );
}

fn benchmark_checked_log_base_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_log_base_2()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.checked_log_base_2()))],
    );
}
