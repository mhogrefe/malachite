use std::fmt::Debug;

use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::named::Named;
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_checked_from_u8);
    register_demo!(registry, demo_u16_checked_from_u8);
    register_demo!(registry, demo_u32_checked_from_u8);
    register_demo!(registry, demo_u64_checked_from_u8);
    register_demo!(registry, demo_u128_checked_from_u8);
    register_demo!(registry, demo_usize_checked_from_u8);
    register_demo!(registry, demo_i8_checked_from_u8);
    register_demo!(registry, demo_i16_checked_from_u8);
    register_demo!(registry, demo_i32_checked_from_u8);
    register_demo!(registry, demo_i64_checked_from_u8);
    register_demo!(registry, demo_i128_checked_from_u8);
    register_demo!(registry, demo_isize_checked_from_u8);
    register_demo!(registry, demo_u8_checked_from_u16);
    register_demo!(registry, demo_u16_checked_from_u16);
    register_demo!(registry, demo_u32_checked_from_u16);
    register_demo!(registry, demo_u64_checked_from_u16);
    register_demo!(registry, demo_u128_checked_from_u16);
    register_demo!(registry, demo_usize_checked_from_u16);
    register_demo!(registry, demo_i8_checked_from_u16);
    register_demo!(registry, demo_i16_checked_from_u16);
    register_demo!(registry, demo_i32_checked_from_u16);
    register_demo!(registry, demo_i64_checked_from_u16);
    register_demo!(registry, demo_i128_checked_from_u16);
    register_demo!(registry, demo_isize_checked_from_u16);
    register_demo!(registry, demo_u8_checked_from_u32);
    register_demo!(registry, demo_u16_checked_from_u32);
    register_demo!(registry, demo_u32_checked_from_u32);
    register_demo!(registry, demo_u64_checked_from_u32);
    register_demo!(registry, demo_u128_checked_from_u32);
    register_demo!(registry, demo_usize_checked_from_u32);
    register_demo!(registry, demo_i8_checked_from_u32);
    register_demo!(registry, demo_i16_checked_from_u32);
    register_demo!(registry, demo_i32_checked_from_u32);
    register_demo!(registry, demo_i64_checked_from_u32);
    register_demo!(registry, demo_i128_checked_from_u32);
    register_demo!(registry, demo_isize_checked_from_u32);
    register_demo!(registry, demo_u8_checked_from_u64);
    register_demo!(registry, demo_u16_checked_from_u64);
    register_demo!(registry, demo_u32_checked_from_u64);
    register_demo!(registry, demo_u64_checked_from_u64);
    register_demo!(registry, demo_u128_checked_from_u64);
    register_demo!(registry, demo_usize_checked_from_u64);
    register_demo!(registry, demo_i8_checked_from_u64);
    register_demo!(registry, demo_i16_checked_from_u64);
    register_demo!(registry, demo_i32_checked_from_u64);
    register_demo!(registry, demo_i64_checked_from_u64);
    register_demo!(registry, demo_i128_checked_from_u64);
    register_demo!(registry, demo_isize_checked_from_u64);
    register_demo!(registry, demo_u8_checked_from_usize);
    register_demo!(registry, demo_u16_checked_from_usize);
    register_demo!(registry, demo_u32_checked_from_usize);
    register_demo!(registry, demo_u64_checked_from_usize);
    register_demo!(registry, demo_u128_checked_from_usize);
    register_demo!(registry, demo_usize_checked_from_usize);
    register_demo!(registry, demo_i8_checked_from_usize);
    register_demo!(registry, demo_i16_checked_from_usize);
    register_demo!(registry, demo_i32_checked_from_usize);
    register_demo!(registry, demo_i64_checked_from_usize);
    register_demo!(registry, demo_i128_checked_from_usize);
    register_demo!(registry, demo_isize_checked_from_usize);
    register_demo!(registry, demo_u8_checked_from_i8);
    register_demo!(registry, demo_u16_checked_from_i8);
    register_demo!(registry, demo_u32_checked_from_i8);
    register_demo!(registry, demo_u64_checked_from_i8);
    register_demo!(registry, demo_u128_checked_from_i8);
    register_demo!(registry, demo_usize_checked_from_i8);
    register_demo!(registry, demo_i8_checked_from_i8);
    register_demo!(registry, demo_i16_checked_from_i8);
    register_demo!(registry, demo_i32_checked_from_i8);
    register_demo!(registry, demo_i64_checked_from_i8);
    register_demo!(registry, demo_i128_checked_from_i8);
    register_demo!(registry, demo_isize_checked_from_i8);
    register_demo!(registry, demo_u8_checked_from_i16);
    register_demo!(registry, demo_u16_checked_from_i16);
    register_demo!(registry, demo_u32_checked_from_i16);
    register_demo!(registry, demo_u64_checked_from_i16);
    register_demo!(registry, demo_u128_checked_from_i16);
    register_demo!(registry, demo_usize_checked_from_i16);
    register_demo!(registry, demo_i8_checked_from_i16);
    register_demo!(registry, demo_i16_checked_from_i16);
    register_demo!(registry, demo_i32_checked_from_i16);
    register_demo!(registry, demo_i64_checked_from_i16);
    register_demo!(registry, demo_i128_checked_from_i16);
    register_demo!(registry, demo_isize_checked_from_i16);
    register_demo!(registry, demo_u8_checked_from_i32);
    register_demo!(registry, demo_u16_checked_from_i32);
    register_demo!(registry, demo_u32_checked_from_i32);
    register_demo!(registry, demo_u64_checked_from_i32);
    register_demo!(registry, demo_u128_checked_from_i32);
    register_demo!(registry, demo_usize_checked_from_i32);
    register_demo!(registry, demo_i8_checked_from_i32);
    register_demo!(registry, demo_i16_checked_from_i32);
    register_demo!(registry, demo_i32_checked_from_i32);
    register_demo!(registry, demo_i64_checked_from_i32);
    register_demo!(registry, demo_i128_checked_from_i32);
    register_demo!(registry, demo_isize_checked_from_i32);
    register_demo!(registry, demo_u8_checked_from_i64);
    register_demo!(registry, demo_u16_checked_from_i64);
    register_demo!(registry, demo_u32_checked_from_i64);
    register_demo!(registry, demo_u64_checked_from_i64);
    register_demo!(registry, demo_u128_checked_from_i64);
    register_demo!(registry, demo_usize_checked_from_i64);
    register_demo!(registry, demo_i8_checked_from_i64);
    register_demo!(registry, demo_i16_checked_from_i64);
    register_demo!(registry, demo_i32_checked_from_i64);
    register_demo!(registry, demo_i64_checked_from_i64);
    register_demo!(registry, demo_i128_checked_from_i64);
    register_demo!(registry, demo_isize_checked_from_i64);
    register_demo!(registry, demo_u8_checked_from_isize);
    register_demo!(registry, demo_u16_checked_from_isize);
    register_demo!(registry, demo_u32_checked_from_isize);
    register_demo!(registry, demo_u64_checked_from_isize);
    register_demo!(registry, demo_u128_checked_from_isize);
    register_demo!(registry, demo_usize_checked_from_isize);
    register_demo!(registry, demo_i8_checked_from_isize);
    register_demo!(registry, demo_i16_checked_from_isize);
    register_demo!(registry, demo_i32_checked_from_isize);
    register_demo!(registry, demo_i64_checked_from_isize);
    register_demo!(registry, demo_i128_checked_from_isize);
    register_demo!(registry, demo_isize_checked_from_isize);
    register_bench!(registry, None, benchmark_u8_checked_from_u8);
    register_bench!(registry, None, benchmark_u16_checked_from_u8);
    register_bench!(registry, None, benchmark_u32_checked_from_u8);
    register_bench!(registry, None, benchmark_u64_checked_from_u8);
    register_bench!(registry, None, benchmark_u128_checked_from_u8);
    register_bench!(registry, None, benchmark_usize_checked_from_u8);
    register_bench!(registry, None, benchmark_i8_checked_from_u8);
    register_bench!(registry, None, benchmark_i16_checked_from_u8);
    register_bench!(registry, None, benchmark_i32_checked_from_u8);
    register_bench!(registry, None, benchmark_i64_checked_from_u8);
    register_bench!(registry, None, benchmark_i128_checked_from_u8);
    register_bench!(registry, None, benchmark_isize_checked_from_u8);
    register_bench!(registry, None, benchmark_u8_checked_from_u16);
    register_bench!(registry, None, benchmark_u16_checked_from_u16);
    register_bench!(registry, None, benchmark_u32_checked_from_u16);
    register_bench!(registry, None, benchmark_u64_checked_from_u16);
    register_bench!(registry, None, benchmark_u128_checked_from_u16);
    register_bench!(registry, None, benchmark_usize_checked_from_u16);
    register_bench!(registry, None, benchmark_i8_checked_from_u16);
    register_bench!(registry, None, benchmark_i16_checked_from_u16);
    register_bench!(registry, None, benchmark_i32_checked_from_u16);
    register_bench!(registry, None, benchmark_i64_checked_from_u16);
    register_bench!(registry, None, benchmark_i128_checked_from_u16);
    register_bench!(registry, None, benchmark_isize_checked_from_u16);
    register_bench!(registry, None, benchmark_u8_checked_from_u32);
    register_bench!(registry, None, benchmark_u16_checked_from_u32);
    register_bench!(registry, None, benchmark_u32_checked_from_u32);
    register_bench!(registry, None, benchmark_u64_checked_from_u32);
    register_bench!(registry, None, benchmark_u128_checked_from_u32);
    register_bench!(registry, None, benchmark_usize_checked_from_u32);
    register_bench!(registry, None, benchmark_i8_checked_from_u32);
    register_bench!(registry, None, benchmark_i16_checked_from_u32);
    register_bench!(registry, None, benchmark_i32_checked_from_u32);
    register_bench!(registry, None, benchmark_i64_checked_from_u32);
    register_bench!(registry, None, benchmark_i128_checked_from_u32);
    register_bench!(registry, None, benchmark_isize_checked_from_u32);
    register_bench!(registry, None, benchmark_u8_checked_from_u64);
    register_bench!(registry, None, benchmark_u16_checked_from_u64);
    register_bench!(registry, None, benchmark_u32_checked_from_u64);
    register_bench!(registry, None, benchmark_u64_checked_from_u64);
    register_bench!(registry, None, benchmark_u128_checked_from_u64);
    register_bench!(registry, None, benchmark_usize_checked_from_u64);
    register_bench!(registry, None, benchmark_i8_checked_from_u64);
    register_bench!(registry, None, benchmark_i16_checked_from_u64);
    register_bench!(registry, None, benchmark_i32_checked_from_u64);
    register_bench!(registry, None, benchmark_i64_checked_from_u64);
    register_bench!(registry, None, benchmark_i128_checked_from_u64);
    register_bench!(registry, None, benchmark_isize_checked_from_u64);
    register_bench!(registry, None, benchmark_u8_checked_from_usize);
    register_bench!(registry, None, benchmark_u16_checked_from_usize);
    register_bench!(registry, None, benchmark_u32_checked_from_usize);
    register_bench!(registry, None, benchmark_u64_checked_from_usize);
    register_bench!(registry, None, benchmark_u128_checked_from_usize);
    register_bench!(registry, None, benchmark_usize_checked_from_usize);
    register_bench!(registry, None, benchmark_i8_checked_from_usize);
    register_bench!(registry, None, benchmark_i16_checked_from_usize);
    register_bench!(registry, None, benchmark_i32_checked_from_usize);
    register_bench!(registry, None, benchmark_i64_checked_from_usize);
    register_bench!(registry, None, benchmark_i128_checked_from_usize);
    register_bench!(registry, None, benchmark_isize_checked_from_usize);
    register_bench!(registry, None, benchmark_u8_checked_from_i8);
    register_bench!(registry, None, benchmark_u16_checked_from_i8);
    register_bench!(registry, None, benchmark_u32_checked_from_i8);
    register_bench!(registry, None, benchmark_u64_checked_from_i8);
    register_bench!(registry, None, benchmark_u128_checked_from_i8);
    register_bench!(registry, None, benchmark_usize_checked_from_i8);
    register_bench!(registry, None, benchmark_i8_checked_from_i8);
    register_bench!(registry, None, benchmark_i16_checked_from_i8);
    register_bench!(registry, None, benchmark_i32_checked_from_i8);
    register_bench!(registry, None, benchmark_i64_checked_from_i8);
    register_bench!(registry, None, benchmark_i128_checked_from_i8);
    register_bench!(registry, None, benchmark_isize_checked_from_i8);
    register_bench!(registry, None, benchmark_u8_checked_from_i16);
    register_bench!(registry, None, benchmark_u16_checked_from_i16);
    register_bench!(registry, None, benchmark_u32_checked_from_i16);
    register_bench!(registry, None, benchmark_u64_checked_from_i16);
    register_bench!(registry, None, benchmark_u128_checked_from_i16);
    register_bench!(registry, None, benchmark_usize_checked_from_i16);
    register_bench!(registry, None, benchmark_i8_checked_from_i16);
    register_bench!(registry, None, benchmark_i16_checked_from_i16);
    register_bench!(registry, None, benchmark_i32_checked_from_i16);
    register_bench!(registry, None, benchmark_i64_checked_from_i16);
    register_bench!(registry, None, benchmark_i128_checked_from_i16);
    register_bench!(registry, None, benchmark_isize_checked_from_i16);
    register_bench!(registry, None, benchmark_u8_checked_from_i32);
    register_bench!(registry, None, benchmark_u16_checked_from_i32);
    register_bench!(registry, None, benchmark_u32_checked_from_i32);
    register_bench!(registry, None, benchmark_u64_checked_from_i32);
    register_bench!(registry, None, benchmark_u128_checked_from_i32);
    register_bench!(registry, None, benchmark_usize_checked_from_i32);
    register_bench!(registry, None, benchmark_i8_checked_from_i32);
    register_bench!(registry, None, benchmark_i16_checked_from_i32);
    register_bench!(registry, None, benchmark_i32_checked_from_i32);
    register_bench!(registry, None, benchmark_i64_checked_from_i32);
    register_bench!(registry, None, benchmark_i128_checked_from_i32);
    register_bench!(registry, None, benchmark_isize_checked_from_i32);
    register_bench!(registry, None, benchmark_u8_checked_from_i64);
    register_bench!(registry, None, benchmark_u16_checked_from_i64);
    register_bench!(registry, None, benchmark_u32_checked_from_i64);
    register_bench!(registry, None, benchmark_u64_checked_from_i64);
    register_bench!(registry, None, benchmark_u128_checked_from_i64);
    register_bench!(registry, None, benchmark_usize_checked_from_i64);
    register_bench!(registry, None, benchmark_i8_checked_from_i64);
    register_bench!(registry, None, benchmark_i16_checked_from_i64);
    register_bench!(registry, None, benchmark_i32_checked_from_i64);
    register_bench!(registry, None, benchmark_i64_checked_from_i64);
    register_bench!(registry, None, benchmark_i128_checked_from_i64);
    register_bench!(registry, None, benchmark_isize_checked_from_i64);
    register_bench!(registry, None, benchmark_u8_checked_from_isize);
    register_bench!(registry, None, benchmark_u16_checked_from_isize);
    register_bench!(registry, None, benchmark_u32_checked_from_isize);
    register_bench!(registry, None, benchmark_u64_checked_from_isize);
    register_bench!(registry, None, benchmark_u128_checked_from_isize);
    register_bench!(registry, None, benchmark_usize_checked_from_isize);
    register_bench!(registry, None, benchmark_i8_checked_from_isize);
    register_bench!(registry, None, benchmark_i16_checked_from_isize);
    register_bench!(registry, None, benchmark_i32_checked_from_isize);
    register_bench!(registry, None, benchmark_i64_checked_from_isize);
    register_bench!(registry, None, benchmark_i128_checked_from_isize);
    register_bench!(registry, None, benchmark_isize_checked_from_isize);
}

fn demo_checked_from_unsigned<T: PrimitiveUnsigned + Rand, U: Debug + Named>(
    gm: GenerationMode,
    limit: usize,
) where
    U: CheckedFrom<T>,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            U::NAME,
            u,
            U::checked_from(u)
        );
    }
}

fn demo_checked_from_signed<T: PrimitiveSigned + Rand, U: Debug + Named>(
    gm: GenerationMode,
    limit: usize,
) where
    U: CheckedFrom<T>,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
{
    for i in signeds::<T>(gm).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            U::NAME,
            i,
            U::checked_from(i)
        );
    }
}

fn benchmark_checked_from_unsigned<T: PrimitiveUnsigned + Rand, U: Named>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    U: CheckedFrom<T>,
{
    m_run_benchmark(
        &format!("{}.checked_from({})", U::NAME, T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(U::checked_from(n))))],
    );
}

fn benchmark_checked_from_signed<T: PrimitiveSigned + Rand, U: Named>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    U: CheckedFrom<T>,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
{
    m_run_benchmark(
        &format!("{}.checked_from({})", U::NAME, T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(U::checked_from(n))))],
    );
}

macro_rules! unsigned {
    ($t:ident, $u: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_checked_from_unsigned::<$t, $u>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_checked_from_unsigned::<$t, $u>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $u: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_checked_from_signed::<$t, $u>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_checked_from_signed::<$t, $u>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    u8,
    demo_u8_checked_from_u8,
    benchmark_u8_checked_from_u8
);
unsigned!(
    u8,
    u16,
    demo_u16_checked_from_u8,
    benchmark_u16_checked_from_u8
);
unsigned!(
    u8,
    u32,
    demo_u32_checked_from_u8,
    benchmark_u32_checked_from_u8
);
unsigned!(
    u8,
    u64,
    demo_u64_checked_from_u8,
    benchmark_u64_checked_from_u8
);
unsigned!(
    u8,
    u128,
    demo_u128_checked_from_u8,
    benchmark_u128_checked_from_u8
);
unsigned!(
    u8,
    usize,
    demo_usize_checked_from_u8,
    benchmark_usize_checked_from_u8
);
unsigned!(
    u8,
    i8,
    demo_i8_checked_from_u8,
    benchmark_i8_checked_from_u8
);
unsigned!(
    u8,
    i16,
    demo_i16_checked_from_u8,
    benchmark_i16_checked_from_u8
);
unsigned!(
    u8,
    i32,
    demo_i32_checked_from_u8,
    benchmark_i32_checked_from_u8
);
unsigned!(
    u8,
    i64,
    demo_i64_checked_from_u8,
    benchmark_i64_checked_from_u8
);
unsigned!(
    u8,
    i128,
    demo_i128_checked_from_u8,
    benchmark_i128_checked_from_u8
);
unsigned!(
    u8,
    isize,
    demo_isize_checked_from_u8,
    benchmark_isize_checked_from_u8
);
unsigned!(
    u16,
    u8,
    demo_u8_checked_from_u16,
    benchmark_u8_checked_from_u16
);
unsigned!(
    u16,
    u16,
    demo_u16_checked_from_u16,
    benchmark_u16_checked_from_u16
);
unsigned!(
    u16,
    u32,
    demo_u32_checked_from_u16,
    benchmark_u32_checked_from_u16
);
unsigned!(
    u16,
    u64,
    demo_u64_checked_from_u16,
    benchmark_u64_checked_from_u16
);
unsigned!(
    u16,
    u128,
    demo_u128_checked_from_u16,
    benchmark_u128_checked_from_u16
);
unsigned!(
    u16,
    usize,
    demo_usize_checked_from_u16,
    benchmark_usize_checked_from_u16
);
unsigned!(
    u16,
    i8,
    demo_i8_checked_from_u16,
    benchmark_i8_checked_from_u16
);
unsigned!(
    u16,
    i16,
    demo_i16_checked_from_u16,
    benchmark_i16_checked_from_u16
);
unsigned!(
    u16,
    i32,
    demo_i32_checked_from_u16,
    benchmark_i32_checked_from_u16
);
unsigned!(
    u16,
    i64,
    demo_i64_checked_from_u16,
    benchmark_i64_checked_from_u16
);
unsigned!(
    u16,
    i128,
    demo_i128_checked_from_u16,
    benchmark_i128_checked_from_u16
);
unsigned!(
    u16,
    isize,
    demo_isize_checked_from_u16,
    benchmark_isize_checked_from_u16
);
unsigned!(
    u32,
    u8,
    demo_u8_checked_from_u32,
    benchmark_u8_checked_from_u32
);
unsigned!(
    u32,
    u16,
    demo_u16_checked_from_u32,
    benchmark_u16_checked_from_u32
);
unsigned!(
    u32,
    u32,
    demo_u32_checked_from_u32,
    benchmark_u32_checked_from_u32
);
unsigned!(
    u32,
    u64,
    demo_u64_checked_from_u32,
    benchmark_u64_checked_from_u32
);
unsigned!(
    u32,
    u128,
    demo_u128_checked_from_u32,
    benchmark_u128_checked_from_u32
);
unsigned!(
    u32,
    usize,
    demo_usize_checked_from_u32,
    benchmark_usize_checked_from_u32
);
unsigned!(
    u32,
    i8,
    demo_i8_checked_from_u32,
    benchmark_i8_checked_from_u32
);
unsigned!(
    u32,
    i16,
    demo_i16_checked_from_u32,
    benchmark_i16_checked_from_u32
);
unsigned!(
    u32,
    i32,
    demo_i32_checked_from_u32,
    benchmark_i32_checked_from_u32
);
unsigned!(
    u32,
    i64,
    demo_i64_checked_from_u32,
    benchmark_i64_checked_from_u32
);
unsigned!(
    u32,
    i128,
    demo_i128_checked_from_u32,
    benchmark_i128_checked_from_u32
);
unsigned!(
    u32,
    isize,
    demo_isize_checked_from_u32,
    benchmark_isize_checked_from_u32
);
unsigned!(
    u64,
    u8,
    demo_u8_checked_from_u64,
    benchmark_u8_checked_from_u64
);
unsigned!(
    u64,
    u16,
    demo_u16_checked_from_u64,
    benchmark_u16_checked_from_u64
);
unsigned!(
    u64,
    u32,
    demo_u32_checked_from_u64,
    benchmark_u32_checked_from_u64
);
unsigned!(
    u64,
    u64,
    demo_u64_checked_from_u64,
    benchmark_u64_checked_from_u64
);
unsigned!(
    u64,
    u128,
    demo_u128_checked_from_u64,
    benchmark_u128_checked_from_u64
);
unsigned!(
    u64,
    usize,
    demo_usize_checked_from_u64,
    benchmark_usize_checked_from_u64
);
unsigned!(
    u64,
    i8,
    demo_i8_checked_from_u64,
    benchmark_i8_checked_from_u64
);
unsigned!(
    u64,
    i16,
    demo_i16_checked_from_u64,
    benchmark_i16_checked_from_u64
);
unsigned!(
    u64,
    i32,
    demo_i32_checked_from_u64,
    benchmark_i32_checked_from_u64
);
unsigned!(
    u64,
    i64,
    demo_i64_checked_from_u64,
    benchmark_i64_checked_from_u64
);
unsigned!(
    u64,
    i128,
    demo_i128_checked_from_u64,
    benchmark_i128_checked_from_u64
);
unsigned!(
    u64,
    isize,
    demo_isize_checked_from_u64,
    benchmark_isize_checked_from_u64
);
unsigned!(
    usize,
    u8,
    demo_u8_checked_from_usize,
    benchmark_u8_checked_from_usize
);
unsigned!(
    usize,
    u16,
    demo_u16_checked_from_usize,
    benchmark_u16_checked_from_usize
);
unsigned!(
    usize,
    u32,
    demo_u32_checked_from_usize,
    benchmark_u32_checked_from_usize
);
unsigned!(
    usize,
    u64,
    demo_u64_checked_from_usize,
    benchmark_u64_checked_from_usize
);
unsigned!(
    usize,
    u128,
    demo_u128_checked_from_usize,
    benchmark_u128_checked_from_usize
);
unsigned!(
    usize,
    usize,
    demo_usize_checked_from_usize,
    benchmark_usize_checked_from_usize
);
unsigned!(
    usize,
    i8,
    demo_i8_checked_from_usize,
    benchmark_i8_checked_from_usize
);
unsigned!(
    usize,
    i16,
    demo_i16_checked_from_usize,
    benchmark_i16_checked_from_usize
);
unsigned!(
    usize,
    i32,
    demo_i32_checked_from_usize,
    benchmark_i32_checked_from_usize
);
unsigned!(
    usize,
    i64,
    demo_i64_checked_from_usize,
    benchmark_i64_checked_from_usize
);
unsigned!(
    usize,
    i128,
    demo_i128_checked_from_usize,
    benchmark_i128_checked_from_usize
);
unsigned!(
    usize,
    isize,
    demo_isize_checked_from_usize,
    benchmark_isize_checked_from_usize
);

signed!(
    i8,
    u8,
    demo_u8_checked_from_i8,
    benchmark_u8_checked_from_i8
);
signed!(
    i8,
    u16,
    demo_u16_checked_from_i8,
    benchmark_u16_checked_from_i8
);
signed!(
    i8,
    u32,
    demo_u32_checked_from_i8,
    benchmark_u32_checked_from_i8
);
signed!(
    i8,
    u64,
    demo_u64_checked_from_i8,
    benchmark_u64_checked_from_i8
);
signed!(
    i8,
    u128,
    demo_u128_checked_from_i8,
    benchmark_u128_checked_from_i8
);
signed!(
    i8,
    usize,
    demo_usize_checked_from_i8,
    benchmark_usize_checked_from_i8
);
signed!(
    i8,
    i8,
    demo_i8_checked_from_i8,
    benchmark_i8_checked_from_i8
);
signed!(
    i8,
    i16,
    demo_i16_checked_from_i8,
    benchmark_i16_checked_from_i8
);
signed!(
    i8,
    i32,
    demo_i32_checked_from_i8,
    benchmark_i32_checked_from_i8
);
signed!(
    i8,
    i64,
    demo_i64_checked_from_i8,
    benchmark_i64_checked_from_i8
);
signed!(
    i8,
    i128,
    demo_i128_checked_from_i8,
    benchmark_i128_checked_from_i8
);
signed!(
    i8,
    isize,
    demo_isize_checked_from_i8,
    benchmark_isize_checked_from_i8
);
signed!(
    i16,
    u8,
    demo_u8_checked_from_i16,
    benchmark_u8_checked_from_i16
);
signed!(
    i16,
    u16,
    demo_u16_checked_from_i16,
    benchmark_u16_checked_from_i16
);
signed!(
    i16,
    u32,
    demo_u32_checked_from_i16,
    benchmark_u32_checked_from_i16
);
signed!(
    i16,
    u64,
    demo_u64_checked_from_i16,
    benchmark_u64_checked_from_i16
);
signed!(
    i16,
    u128,
    demo_u128_checked_from_i16,
    benchmark_u128_checked_from_i16
);
signed!(
    i16,
    usize,
    demo_usize_checked_from_i16,
    benchmark_usize_checked_from_i16
);
signed!(
    i16,
    i8,
    demo_i8_checked_from_i16,
    benchmark_i8_checked_from_i16
);
signed!(
    i16,
    i16,
    demo_i16_checked_from_i16,
    benchmark_i16_checked_from_i16
);
signed!(
    i16,
    i32,
    demo_i32_checked_from_i16,
    benchmark_i32_checked_from_i16
);
signed!(
    i16,
    i64,
    demo_i64_checked_from_i16,
    benchmark_i64_checked_from_i16
);
signed!(
    i16,
    i128,
    demo_i128_checked_from_i16,
    benchmark_i128_checked_from_i16
);
signed!(
    i16,
    isize,
    demo_isize_checked_from_i16,
    benchmark_isize_checked_from_i16
);
signed!(
    i32,
    u8,
    demo_u8_checked_from_i32,
    benchmark_u8_checked_from_i32
);
signed!(
    i32,
    u16,
    demo_u16_checked_from_i32,
    benchmark_u16_checked_from_i32
);
signed!(
    i32,
    u32,
    demo_u32_checked_from_i32,
    benchmark_u32_checked_from_i32
);
signed!(
    i32,
    u64,
    demo_u64_checked_from_i32,
    benchmark_u64_checked_from_i32
);
signed!(
    i32,
    u128,
    demo_u128_checked_from_i32,
    benchmark_u128_checked_from_i32
);
signed!(
    i32,
    usize,
    demo_usize_checked_from_i32,
    benchmark_usize_checked_from_i32
);
signed!(
    i32,
    i8,
    demo_i8_checked_from_i32,
    benchmark_i8_checked_from_i32
);
signed!(
    i32,
    i16,
    demo_i16_checked_from_i32,
    benchmark_i16_checked_from_i32
);
signed!(
    i32,
    i32,
    demo_i32_checked_from_i32,
    benchmark_i32_checked_from_i32
);
signed!(
    i32,
    i64,
    demo_i64_checked_from_i32,
    benchmark_i64_checked_from_i32
);
signed!(
    i32,
    i128,
    demo_i128_checked_from_i32,
    benchmark_i128_checked_from_i32
);
signed!(
    i32,
    isize,
    demo_isize_checked_from_i32,
    benchmark_isize_checked_from_i32
);
signed!(
    i64,
    u8,
    demo_u8_checked_from_i64,
    benchmark_u8_checked_from_i64
);
signed!(
    i64,
    u16,
    demo_u16_checked_from_i64,
    benchmark_u16_checked_from_i64
);
signed!(
    i64,
    u32,
    demo_u32_checked_from_i64,
    benchmark_u32_checked_from_i64
);
signed!(
    i64,
    u64,
    demo_u64_checked_from_i64,
    benchmark_u64_checked_from_i64
);
signed!(
    i64,
    u128,
    demo_u128_checked_from_i64,
    benchmark_u128_checked_from_i64
);
signed!(
    i64,
    usize,
    demo_usize_checked_from_i64,
    benchmark_usize_checked_from_i64
);
signed!(
    i64,
    i8,
    demo_i8_checked_from_i64,
    benchmark_i8_checked_from_i64
);
signed!(
    i64,
    i16,
    demo_i16_checked_from_i64,
    benchmark_i16_checked_from_i64
);
signed!(
    i64,
    i32,
    demo_i32_checked_from_i64,
    benchmark_i32_checked_from_i64
);
signed!(
    i64,
    i64,
    demo_i64_checked_from_i64,
    benchmark_i64_checked_from_i64
);
signed!(
    i64,
    i128,
    demo_i128_checked_from_i64,
    benchmark_i128_checked_from_i64
);
signed!(
    i64,
    isize,
    demo_isize_checked_from_i64,
    benchmark_isize_checked_from_i64
);
signed!(
    isize,
    u8,
    demo_u8_checked_from_isize,
    benchmark_u8_checked_from_isize
);
signed!(
    isize,
    u16,
    demo_u16_checked_from_isize,
    benchmark_u16_checked_from_isize
);
signed!(
    isize,
    u32,
    demo_u32_checked_from_isize,
    benchmark_u32_checked_from_isize
);
signed!(
    isize,
    u64,
    demo_u64_checked_from_isize,
    benchmark_u64_checked_from_isize
);
signed!(
    isize,
    u128,
    demo_u128_checked_from_isize,
    benchmark_u128_checked_from_isize
);
signed!(
    isize,
    usize,
    demo_usize_checked_from_isize,
    benchmark_usize_checked_from_isize
);
signed!(
    isize,
    i8,
    demo_i8_checked_from_isize,
    benchmark_i8_checked_from_isize
);
signed!(
    isize,
    i16,
    demo_i16_checked_from_isize,
    benchmark_i16_checked_from_isize
);
signed!(
    isize,
    i32,
    demo_i32_checked_from_isize,
    benchmark_i32_checked_from_isize
);
signed!(
    isize,
    i64,
    demo_i64_checked_from_isize,
    benchmark_i64_checked_from_isize
);
signed!(
    isize,
    i128,
    demo_i128_checked_from_isize,
    benchmark_i128_checked_from_isize
);
signed!(
    isize,
    isize,
    demo_isize_checked_from_isize,
    benchmark_isize_checked_from_isize
);
