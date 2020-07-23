use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u16_split_in_half);
    register_demo!(registry, demo_u32_split_in_half);
    register_demo!(registry, demo_u64_split_in_half);
    register_bench!(registry, None, benchmark_u16_split_in_half);
    register_bench!(registry, None, benchmark_u32_split_in_half);
    register_bench!(registry, None, benchmark_u64_split_in_half);
}

fn demo_unsigned_split_in_half<T: PrimitiveUnsigned + SplitInHalf + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.split_in_half() = {:?}", u, u.split_in_half());
    }
}

fn benchmark_unsigned_split_in_half<T: PrimitiveUnsigned + Rand + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.split_in_half()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| usize::exact_from(n.significant_bits())),
        "index",
        &mut [("malachite", &mut (|n| no_out!(n.split_in_half())))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_split_in_half::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_split_in_half::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u16, demo_u16_split_in_half, benchmark_u16_split_in_half);
unsigned!(u32, demo_u32_split_in_half, benchmark_u32_split_in_half);
unsigned!(u64, demo_u64_split_in_half, benchmark_u64_split_in_half);
