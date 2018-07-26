use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::unsigneds;
use malachite_base::num::{PrimitiveUnsigned, SplitInHalf};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u16_upper_half);
    register_demo!(registry, demo_u32_upper_half);
    register_demo!(registry, demo_u64_upper_half);
    register_bench!(registry, None, benchmark_u16_upper_half);
    register_bench!(registry, None, benchmark_u32_upper_half);
    register_bench!(registry, None, benchmark_u64_upper_half);
}

fn demo_unsigned_upper_half<T: PrimitiveUnsigned + SplitInHalf>(gm: GenerationMode, limit: usize)
where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.upper_half() = {}", u, u.upper_half());
    }
}

fn benchmark_unsigned_upper_half<T: PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    m_run_benchmark(
        &format!("{}.upper_half()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.upper_half())))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_upper_half::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_upper_half::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u16, demo_u16_upper_half, benchmark_u16_upper_half);
unsigned!(u32, demo_u32_upper_half, benchmark_u32_upper_half);
unsigned!(u64, demo_u64_upper_half, benchmark_u64_upper_half);
