use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SplitInHalf;

use malachite_base_test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_generic_demos!(runner, demo_unsigned_upper_half, u16, u32, u64, u128);
    register_generic_benches!(runner, benchmark_unsigned_upper_half, u16, u32, u64, u128);
}

fn demo_unsigned_upper_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.upper_half() = {}", u, u.upper_half());
    }
}

fn benchmark_unsigned_upper_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.upper_half()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.upper_half()))],
    );
}
