use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_pair_gen_var_2,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_divisible_by_power_of_two_unsigned);
    register_signed_demos!(runner, demo_divisible_by_power_of_two_signed);

    register_unsigned_benches!(runner, benchmark_divisible_by_power_of_two_unsigned);
    register_signed_benches!(runner, benchmark_divisible_by_power_of_two_signed);
}

fn demo_divisible_by_power_of_two_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (u, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        if u.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", u, pow);
        } else {
            println!("{} is not divisible by 2^{}", u, pow);
        }
    }
}

fn demo_divisible_by_power_of_two_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (i, pow) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        if i.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", i, pow);
        } else {
            println!("{} is not divisible by 2^{}", i, pow);
        }
    }
}

fn benchmark_divisible_by_power_of_two_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.divisible_by_power_of_two(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.divisible_by_power_of_two(y))
        })],
    );
}

fn benchmark_divisible_by_power_of_two_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.divisible_by_power_of_two(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("i"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.divisible_by_power_of_two(y))
        })],
    );
}
