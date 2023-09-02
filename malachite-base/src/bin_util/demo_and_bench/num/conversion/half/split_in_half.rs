use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_base::test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_generic_demos!(runner, demo_split_in_half, u16, u32, u64, u128);
    register_generic_benches!(runner, benchmark_split_in_half, u16, u32, u64, u128);
}

fn demo_split_in_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.split_in_half() = {:?}", u, u.split_in_half());
    }
}

fn benchmark_split_in_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.split_in_half()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.split_in_half()))],
    );
}
