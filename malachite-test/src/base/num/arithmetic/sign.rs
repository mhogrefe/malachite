use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_sign);
    register_demo!(registry, demo_u16_sign);
    register_demo!(registry, demo_u32_sign);
    register_demo!(registry, demo_u64_sign);
    register_demo!(registry, demo_usize_sign);
    register_demo!(registry, demo_i8_sign);
    register_demo!(registry, demo_i16_sign);
    register_demo!(registry, demo_i32_sign);
    register_demo!(registry, demo_i64_sign);
    register_demo!(registry, demo_isize_sign);
    register_bench!(registry, None, benchmark_u8_sign);
    register_bench!(registry, None, benchmark_u16_sign);
    register_bench!(registry, None, benchmark_u32_sign);
    register_bench!(registry, None, benchmark_u64_sign);
    register_bench!(registry, None, benchmark_usize_sign);
    register_bench!(registry, None, benchmark_i8_sign);
    register_bench!(registry, None, benchmark_i16_sign);
    register_bench!(registry, None, benchmark_i32_sign);
    register_bench!(registry, None, benchmark_i64_sign);
    register_bench!(registry, None, benchmark_isize_sign);
}

fn demo_unsigned_sign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.sign() = {:?}", u, u.sign());
    }
}

fn demo_signed_sign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("{}.sign() = {:?}", i, i.sign());
    }
}

fn benchmark_unsigned_sign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.sign()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::wrapping_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|u| no_out!(u.sign())))],
    );
}

fn benchmark_signed_sign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.sign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|i| no_out!(i.sign())))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_sign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_sign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_sign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_sign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_sign, benchmark_u8_sign);
unsigned!(u16, demo_u16_sign, benchmark_u16_sign);
unsigned!(u32, demo_u32_sign, benchmark_u32_sign);
unsigned!(u64, demo_u64_sign, benchmark_u64_sign);
unsigned!(usize, demo_usize_sign, benchmark_usize_sign);

signed!(i8, demo_i8_sign, benchmark_i8_sign);
signed!(i16, demo_i16_sign, benchmark_i16_sign);
signed!(i32, demo_i32_sign, benchmark_i32_sign);
signed!(i64, demo_i64_sign, benchmark_i64_sign);
signed!(isize, demo_isize_sign, benchmark_isize_sign);
