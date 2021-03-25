use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::JoinHalves;
use malachite_base_test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_generic_demos!(runner, demo_join_halves, u16, u32, u64, u128);
    register_generic_benches!(runner, benchmark_join_halves, u16, u32, u64, u128);
}

fn demo_join_halves<T: JoinHalves + PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    T::Half: PrimitiveUnsigned,
{
    for (x, y) in unsigned_pair_gen::<T::Half>().get(gm, &config).take(limit) {
        println!(
            "{}::join_halves({}, {}) = {}",
            T::NAME,
            x,
            y,
            T::join_halves(x, y)
        );
    }
}

fn benchmark_join_halves<T: JoinHalves + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    run_benchmark(
        &format!(
            "{}::join_halves({}, {})",
            T::NAME,
            T::Half::NAME,
            T::Half::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen::<T::Half>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(T::join_halves(x, y)))],
    );
}
