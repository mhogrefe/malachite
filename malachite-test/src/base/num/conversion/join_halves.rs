use std::cmp::max;

use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u16_join_halves);
    register_demo!(registry, demo_u32_join_halves);
    register_demo!(registry, demo_u64_join_halves);
    register_demo!(registry, demo_u128_join_halves);
    register_bench!(registry, None, benchmark_u16_join_halves);
    register_bench!(registry, None, benchmark_u32_join_halves);
    register_bench!(registry, None, benchmark_u64_join_halves);
    register_bench!(registry, None, benchmark_u128_join_halves);
}

fn demo_unsigned_join_halves<T: JoinHalves + PrimitiveUnsigned>(gm: GenerationMode, limit: usize)
where
    T::Half: PrimitiveUnsigned + Rand,
{
    for (x, y) in pairs_of_unsigneds::<T::Half>(gm).take(limit) {
        println!(
            "{}::join_halves({}, {}) = {}",
            T::NAME,
            x,
            y,
            T::join_halves(x, y)
        );
    }
}

fn benchmark_unsigned_join_halves<T: JoinHalves + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned + Rand,
{
    run_benchmark_old(
        &format!(
            "{}::join_halves({}, {})",
            T::NAME,
            T::Half::NAME,
            T::Half::NAME
        ),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T::Half>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(T::join_halves(x, y))))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_join_halves::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_join_halves::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u16, demo_u16_join_halves, benchmark_u16_join_halves);
unsigned!(u32, demo_u32_join_halves, benchmark_u32_join_halves);
unsigned!(u64, demo_u64_join_halves, benchmark_u64_join_halves);
unsigned!(u128, demo_u128_join_halves, benchmark_u128_join_halves);
