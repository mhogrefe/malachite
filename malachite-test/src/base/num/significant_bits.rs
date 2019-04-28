use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds, unsigneds};
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_significant_bits);
    register_demo!(registry, demo_u16_significant_bits);
    register_demo!(registry, demo_limb_significant_bits);
    register_demo!(registry, demo_u64_significant_bits);
    register_demo!(registry, demo_i8_significant_bits);
    register_demo!(registry, demo_i16_significant_bits);
    register_demo!(registry, demo_signed_limb_significant_bits);
    register_demo!(registry, demo_i64_significant_bits);
    register_bench!(registry, None, benchmark_u8_significant_bits);
    register_bench!(registry, None, benchmark_u16_significant_bits);
    register_bench!(registry, None, benchmark_limb_significant_bits);
    register_bench!(registry, None, benchmark_u64_significant_bits);
    register_bench!(registry, None, benchmark_i8_significant_bits);
    register_bench!(registry, None, benchmark_i16_significant_bits);
    register_bench!(registry, None, benchmark_signed_limb_significant_bits);
    register_bench!(registry, None, benchmark_i64_significant_bits);
}

fn demo_unsigned_significant_bits<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for n in unsigneds::<T>(gm).take(limit) {
        println!("{}.significant_bits() = {}", n, n.significant_bits());
    }
}

fn demo_signed_significant_bits<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
{
    for n in signeds::<T>(gm).take(limit) {
        println!("{}.significant_bits() = {}", n, n.significant_bits());
    }
}

fn benchmark_unsigned_significant_bits<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.significant_bits()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &mut [("malachite", &mut (|n| no_out!(n.significant_bits())))],
    );
}

fn benchmark_signed_significant_bits<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
{
    m_run_benchmark(
        &format!("{}.significant_bits()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &mut [("malachite", &mut (|n| no_out!(n.significant_bits())))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_significant_bits::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_significant_bits::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_significant_bits::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_significant_bits::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_significant_bits, benchmark_u8_significant_bits);
unsigned!(
    u16,
    demo_u16_significant_bits,
    benchmark_u16_significant_bits
);
unsigned!(
    u32,
    demo_limb_significant_bits,
    benchmark_limb_significant_bits
);
unsigned!(
    u64,
    demo_u64_significant_bits,
    benchmark_u64_significant_bits
);

signed!(i8, demo_i8_significant_bits, benchmark_i8_significant_bits);
signed!(
    i16,
    demo_i16_significant_bits,
    benchmark_i16_significant_bits
);
signed!(
    i32,
    demo_signed_limb_significant_bits,
    benchmark_signed_limb_significant_bits
);
signed!(
    i64,
    demo_i64_significant_bits,
    benchmark_i64_significant_bits
);
