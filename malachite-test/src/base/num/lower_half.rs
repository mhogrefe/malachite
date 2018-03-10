use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::unsigneds;
use malachite_base::num::{PrimitiveUnsigned, SplitInHalf};

fn demo_unsigned_lower_half<T: 'static + PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.lower_half() = {}", u, u.lower_half());
    }
}

fn benchmark_unsigned_lower_half<T: 'static + PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    m_run_benchmark(
        &format!("{}.lower_half()", T::NAME),
        BenchmarkType::Ordinary,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|n| no_out!(n.lower_half())))],
    );
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_lower_half::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_lower_half::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u16, demo_u16_lower_half, benchmark_u16_lower_half);
unsigned!(u32, demo_u32_lower_half, benchmark_u32_lower_half);
unsigned!(u64, demo_u64_lower_half, benchmark_u64_lower_half);
