use malachite_base::conversion::WrappingFrom;
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_get_highest_bit);
    register_demo!(registry, demo_u16_get_highest_bit);
    register_demo!(registry, demo_u32_get_highest_bit);
    register_demo!(registry, demo_u64_get_highest_bit);
    register_demo!(registry, demo_usize_get_highest_bit);
    register_demo!(registry, demo_i8_get_highest_bit);
    register_demo!(registry, demo_i16_get_highest_bit);
    register_demo!(registry, demo_i32_get_highest_bit);
    register_demo!(registry, demo_i64_get_highest_bit);
    register_demo!(registry, demo_isize_get_highest_bit);
    register_bench!(registry, None, benchmark_u8_get_highest_bit);
    register_bench!(registry, None, benchmark_u16_get_highest_bit);
    register_bench!(registry, None, benchmark_u32_get_highest_bit);
    register_bench!(registry, None, benchmark_u64_get_highest_bit);
    register_bench!(registry, None, benchmark_usize_get_highest_bit);
    register_bench!(registry, None, benchmark_i8_get_highest_bit);
    register_bench!(registry, None, benchmark_i16_get_highest_bit);
    register_bench!(registry, None, benchmark_i32_get_highest_bit);
    register_bench!(registry, None, benchmark_i64_get_highest_bit);
    register_bench!(registry, None, benchmark_isize_get_highest_bit);
}

fn demo_unsigned_get_highest_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.get_highest_bit() = {}", u, u.get_highest_bit());
    }
}

fn demo_signed_get_highest_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("{}.get_highest_bit() = {}", i, i.get_highest_bit());
    }
}

fn benchmark_unsigned_get_highest_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.get_highest_bit()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::wrapping_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|u| no_out!(u.get_highest_bit())))],
    );
}

fn benchmark_signed_get_highest_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.get_highest_bit()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|i| no_out!(i.get_highest_bit())))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_get_highest_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_get_highest_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_get_highest_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_get_highest_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_get_highest_bit, benchmark_u8_get_highest_bit);
unsigned!(u16, demo_u16_get_highest_bit, benchmark_u16_get_highest_bit);
unsigned!(u32, demo_u32_get_highest_bit, benchmark_u32_get_highest_bit);
unsigned!(u64, demo_u64_get_highest_bit, benchmark_u64_get_highest_bit);
unsigned!(
    usize,
    demo_usize_get_highest_bit,
    benchmark_usize_get_highest_bit
);

signed!(i8, demo_i8_get_highest_bit, benchmark_i8_get_highest_bit);
signed!(i16, demo_i16_get_highest_bit, benchmark_i16_get_highest_bit);
signed!(i32, demo_i32_get_highest_bit, benchmark_i32_get_highest_bit);
signed!(i64, demo_i64_get_highest_bit, benchmark_i64_get_highest_bit);
signed!(
    isize,
    demo_isize_get_highest_bit,
    benchmark_isize_get_highest_bit
);
