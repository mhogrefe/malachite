use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Assign, CheckedFrom, WrappingFrom};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_signed_and_signed, pairs_of_signed_and_unsigned, pairs_of_unsigned_and_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_assign_u8);
    register_demo!(registry, demo_u16_assign_u8);
    register_demo!(registry, demo_u16_assign_u16);
    register_demo!(registry, demo_u32_assign_u8);
    register_demo!(registry, demo_u32_assign_u16);
    register_demo!(registry, demo_u32_assign_u32);
    register_demo!(registry, demo_u64_assign_u8);
    register_demo!(registry, demo_u64_assign_u16);
    register_demo!(registry, demo_u64_assign_u32);
    register_demo!(registry, demo_u64_assign_u64);
    register_demo!(registry, demo_usize_assign_u8);
    register_demo!(registry, demo_usize_assign_u16);

    register_demo!(registry, demo_i8_assign_i8);
    register_demo!(registry, demo_i16_assign_i8);
    register_demo!(registry, demo_i16_assign_i16);
    register_demo!(registry, demo_i32_assign_i8);
    register_demo!(registry, demo_i32_assign_i16);
    register_demo!(registry, demo_i32_assign_i32);
    register_demo!(registry, demo_i64_assign_i8);
    register_demo!(registry, demo_i64_assign_i16);
    register_demo!(registry, demo_i64_assign_i32);
    register_demo!(registry, demo_i64_assign_i64);
    register_demo!(registry, demo_isize_assign_i8);
    register_demo!(registry, demo_isize_assign_i16);

    register_demo!(registry, demo_i16_assign_u8);
    register_demo!(registry, demo_i32_assign_u8);
    register_demo!(registry, demo_i32_assign_u16);
    register_demo!(registry, demo_i64_assign_u8);
    register_demo!(registry, demo_i64_assign_u16);
    register_demo!(registry, demo_i64_assign_u32);
    register_demo!(registry, demo_isize_assign_u8);

    register_bench!(registry, None, benchmark_u8_assign_u8);
    register_bench!(registry, None, benchmark_u16_assign_u8);
    register_bench!(registry, None, benchmark_u16_assign_u16);
    register_bench!(registry, None, benchmark_u32_assign_u8);
    register_bench!(registry, None, benchmark_u32_assign_u16);
    register_bench!(registry, None, benchmark_u32_assign_u32);
    register_bench!(registry, None, benchmark_u64_assign_u8);
    register_bench!(registry, None, benchmark_u64_assign_u16);
    register_bench!(registry, None, benchmark_u64_assign_u32);
    register_bench!(registry, None, benchmark_u64_assign_u64);
    register_bench!(registry, None, benchmark_usize_assign_u8);
    register_bench!(registry, None, benchmark_usize_assign_u16);

    register_bench!(registry, None, benchmark_i8_assign_i8);
    register_bench!(registry, None, benchmark_i16_assign_i8);
    register_bench!(registry, None, benchmark_i16_assign_i16);
    register_bench!(registry, None, benchmark_i32_assign_i8);
    register_bench!(registry, None, benchmark_i32_assign_i16);
    register_bench!(registry, None, benchmark_i32_assign_i32);
    register_bench!(registry, None, benchmark_i64_assign_i8);
    register_bench!(registry, None, benchmark_i64_assign_i16);
    register_bench!(registry, None, benchmark_i64_assign_i32);
    register_bench!(registry, None, benchmark_i64_assign_i64);
    register_bench!(registry, None, benchmark_isize_assign_i8);
    register_bench!(registry, None, benchmark_isize_assign_i16);

    register_bench!(registry, None, benchmark_i16_assign_u8);
    register_bench!(registry, None, benchmark_i32_assign_u8);
    register_bench!(registry, None, benchmark_i32_assign_u16);
    register_bench!(registry, None, benchmark_i64_assign_u8);
    register_bench!(registry, None, benchmark_i64_assign_u16);
    register_bench!(registry, None, benchmark_i64_assign_u32);
    register_bench!(registry, None, benchmark_isize_assign_u8);
}

fn demo_unsigned_assign_unsigned<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: Assign<U>,
{
    for (t, u) in pairs_of_unsigned_and_unsigned::<T, U>(gm).take(limit) {
        let original_t = t;
        let mut t = t;
        t.assign(u);
        println!("t = {}; t::assign({}); t = {}", original_t, u, t,);
    }
}

fn demo_signed_assign_signed<T: PrimitiveSigned + Rand, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: Assign<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (t, u) in pairs_of_signed_and_signed::<T, U>(gm).take(limit) {
        let original_t = t;
        let mut t = t;
        t.assign(u);
        println!("t = {}; t::assign({}); t = {}", original_t, u, t,);
    }
}

fn demo_signed_assign_unsigned<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: Assign<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (t, u) in pairs_of_signed_and_unsigned::<T, U>(gm).take(limit) {
        let original_t = t;
        let mut t = t;
        t.assign(u);
        println!("t = {}; t::assign({}); t = {}", original_t, u, t,);
    }
}

fn benchmark_unsigned_assign_unsigned<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: Assign<U>,
{
    m_run_benchmark(
        &format!("{}.assign({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_unsigned::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(t, _)| usize::checked_from(t.significant_bits()).unwrap()),
        "u.significant_bits()",
        &mut [("malachite", &mut (|(mut t, u)| t.assign(u)))],
    );
}

fn benchmark_signed_assign_signed<T: PrimitiveSigned + Rand, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: Assign<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.assign({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_signed::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(t, _)| usize::checked_from(t.significant_bits()).unwrap()),
        "u.significant_bits()",
        &mut [("malachite", &mut (|(mut t, u)| t.assign(u)))],
    );
}

fn benchmark_signed_assign_unsigned<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: Assign<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.assign({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_unsigned::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(t, _)| usize::checked_from(t.significant_bits()).unwrap()),
        "u.significant_bits()",
        &mut [("malachite", &mut (|(mut t, u)| t.assign(u)))],
    );
}

macro_rules! unsigned_unsigned {
    ($t:ident, $u: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_assign_unsigned::<$t, $u>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_assign_unsigned::<$t, $u>(gm, limit, file_name);
        }
    };
}

macro_rules! signed_signed {
    ($t:ident, $u: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_assign_signed::<$t, $u>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_assign_signed::<$t, $u>(gm, limit, file_name);
        }
    };
}

macro_rules! signed_unsigned {
    ($t:ident, $u: ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_assign_unsigned::<$t, $u>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_assign_unsigned::<$t, $u>(gm, limit, file_name);
        }
    };
}

unsigned_unsigned!(u8, u8, demo_u8_assign_u8, benchmark_u8_assign_u8);
unsigned_unsigned!(u16, u8, demo_u16_assign_u8, benchmark_u16_assign_u8);
unsigned_unsigned!(u16, u16, demo_u16_assign_u16, benchmark_u16_assign_u16);
unsigned_unsigned!(u32, u8, demo_u32_assign_u8, benchmark_u32_assign_u8);
unsigned_unsigned!(u32, u16, demo_u32_assign_u16, benchmark_u32_assign_u16);
unsigned_unsigned!(u32, u32, demo_u32_assign_u32, benchmark_u32_assign_u32);
unsigned_unsigned!(u64, u8, demo_u64_assign_u8, benchmark_u64_assign_u8);
unsigned_unsigned!(u64, u16, demo_u64_assign_u16, benchmark_u64_assign_u16);
unsigned_unsigned!(u64, u32, demo_u64_assign_u32, benchmark_u64_assign_u32);
unsigned_unsigned!(u64, u64, demo_u64_assign_u64, benchmark_u64_assign_u64);
unsigned_unsigned!(usize, u8, demo_usize_assign_u8, benchmark_usize_assign_u8);
unsigned_unsigned!(
    usize,
    u16,
    demo_usize_assign_u16,
    benchmark_usize_assign_u16
);

signed_signed!(i8, i8, demo_i8_assign_i8, benchmark_i8_assign_i8);
signed_signed!(i16, i8, demo_i16_assign_i8, benchmark_i16_assign_i8);
signed_signed!(i16, i16, demo_i16_assign_i16, benchmark_i16_assign_i16);
signed_signed!(i32, i8, demo_i32_assign_i8, benchmark_i32_assign_i8);
signed_signed!(i32, i16, demo_i32_assign_i16, benchmark_i32_assign_i16);
signed_signed!(i32, i32, demo_i32_assign_i32, benchmark_i32_assign_i32);
signed_signed!(i64, i8, demo_i64_assign_i8, benchmark_i64_assign_i8);
signed_signed!(i64, i16, demo_i64_assign_i16, benchmark_i64_assign_i16);
signed_signed!(i64, i32, demo_i64_assign_i32, benchmark_i64_assign_i32);
signed_signed!(i64, i64, demo_i64_assign_i64, benchmark_i64_assign_i64);
signed_signed!(isize, i8, demo_isize_assign_i8, benchmark_isize_assign_i8);
signed_signed!(
    isize,
    i16,
    demo_isize_assign_i16,
    benchmark_isize_assign_i16
);

signed_unsigned!(i16, u8, demo_i16_assign_u8, benchmark_i16_assign_u8);
signed_unsigned!(i32, u8, demo_i32_assign_u8, benchmark_i32_assign_u8);
signed_unsigned!(i32, u16, demo_i32_assign_u16, benchmark_i32_assign_u16);
signed_unsigned!(i64, u8, demo_i64_assign_u8, benchmark_i64_assign_u8);
signed_unsigned!(i64, u16, demo_i64_assign_u16, benchmark_i64_assign_u16);
signed_unsigned!(i64, u32, demo_i64_assign_u32, benchmark_i64_assign_u32);
signed_unsigned!(isize, u8, demo_isize_assign_u8, benchmark_isize_assign_u8);
