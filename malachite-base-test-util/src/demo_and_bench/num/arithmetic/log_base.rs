use malachite_base::num::arithmetic::log_base::{
    _ceiling_log_base_naive, _checked_log_base_naive, _floor_log_base_naive,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_24;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_log_base);
    register_unsigned_demos!(runner, demo_ceiling_log_base);
    register_unsigned_demos!(runner, demo_checked_log_base);
    register_unsigned_benches!(runner, benchmark_floor_log_base_algorithms);
    register_unsigned_benches!(runner, benchmark_ceiling_log_base_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_log_base_algorithms);
}

fn demo_floor_log_base<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, b) in unsigned_pair_gen_var_24::<T, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.floor_log_base({}) = {}", n, b, n.floor_log_base(b));
    }
}

fn demo_ceiling_log_base<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, b) in unsigned_pair_gen_var_24::<T, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.ceiling_log_base({}) = {}", n, b, n.ceiling_log_base(b));
    }
}

fn demo_checked_log_base<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, b) in unsigned_pair_gen_var_24::<T, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.checked_log_base({}) = {:?}",
            n,
            b,
            n.checked_log_base(b)
        );
    }
}

fn benchmark_floor_log_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_log_base({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_24::<T, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, base)| no_out!(n.floor_log_base(base))),
            ("naive", &mut |(n, base)| {
                no_out!(_floor_log_base_naive(n, base))
            }),
        ],
    );
}

fn benchmark_ceiling_log_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_log_base({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_24::<T, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (
                "default",
                &mut |(n, base)| no_out!(n.ceiling_log_base(base)),
            ),
            ("naive", &mut |(n, base)| {
                no_out!(_ceiling_log_base_naive(n, base))
            }),
        ],
    );
}

fn benchmark_checked_log_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_log_base({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_24::<T, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (
                "default",
                &mut |(n, base)| no_out!(n.checked_log_base(base)),
            ),
            ("naive", &mut |(n, base)| {
                no_out!(_checked_log_base_naive(n, base))
            }),
        ],
    );
}
