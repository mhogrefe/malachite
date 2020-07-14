use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_even);
    register_demo!(registry, demo_u16_even);
    register_demo!(registry, demo_u32_even);
    register_demo!(registry, demo_u64_even);
    register_demo!(registry, demo_usize_even);
    register_demo!(registry, demo_i8_even);
    register_demo!(registry, demo_i16_even);
    register_demo!(registry, demo_i32_even);
    register_demo!(registry, demo_i64_even);
    register_demo!(registry, demo_isize_even);

    register_demo!(registry, demo_u8_odd);
    register_demo!(registry, demo_u16_odd);
    register_demo!(registry, demo_u32_odd);
    register_demo!(registry, demo_u64_odd);
    register_demo!(registry, demo_usize_odd);
    register_demo!(registry, demo_i8_odd);
    register_demo!(registry, demo_i16_odd);
    register_demo!(registry, demo_i32_odd);
    register_demo!(registry, demo_i64_odd);
    register_demo!(registry, demo_isize_odd);

    register_bench!(registry, None, benchmark_u8_even);
    register_bench!(registry, None, benchmark_u16_even);
    register_bench!(registry, None, benchmark_u32_even);
    register_bench!(registry, None, benchmark_u64_even);
    register_bench!(registry, None, benchmark_usize_even);
    register_bench!(registry, None, benchmark_i8_even);
    register_bench!(registry, None, benchmark_i16_even);
    register_bench!(registry, None, benchmark_i32_even);
    register_bench!(registry, None, benchmark_i64_even);
    register_bench!(registry, None, benchmark_isize_even);

    register_bench!(registry, None, benchmark_u8_odd);
    register_bench!(registry, None, benchmark_u16_odd);
    register_bench!(registry, None, benchmark_u32_odd);
    register_bench!(registry, None, benchmark_u64_odd);
    register_bench!(registry, None, benchmark_usize_odd);
    register_bench!(registry, None, benchmark_i8_odd);
    register_bench!(registry, None, benchmark_i16_odd);
    register_bench!(registry, None, benchmark_i32_odd);
    register_bench!(registry, None, benchmark_i64_odd);
    register_bench!(registry, None, benchmark_isize_odd);
}

fn demo_unsigned_even<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        if u.even() {
            println!("{} is even", u);
        } else {
            println!("{} is not even", u);
        }
    }
}

fn demo_signed_even<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        if i.even() {
            println!("{} is even", i);
        } else {
            println!("{} is not even", i);
        }
    }
}

fn benchmark_unsigned_even<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.even()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|u| no_out!(u.even())))],
    );
}

fn benchmark_signed_even<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.even()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|i| no_out!(i.even())))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_even::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_even::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_even::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_even::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_even, benchmark_u8_even);
unsigned!(u16, demo_u16_even, benchmark_u16_even);
unsigned!(u32, demo_u32_even, benchmark_u32_even);
unsigned!(u64, demo_u64_even, benchmark_u64_even);
unsigned!(usize, demo_usize_even, benchmark_usize_even);

signed!(i8, demo_i8_even, benchmark_i8_even);
signed!(i16, demo_i16_even, benchmark_i16_even);
signed!(i32, demo_i32_even, benchmark_i32_even);
signed!(i64, demo_i64_even, benchmark_i64_even);
signed!(isize, demo_isize_even, benchmark_isize_even);

fn demo_unsigned_odd<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        if u.odd() {
            println!("{} is odd", u);
        } else {
            println!("{} is not odd", u);
        }
    }
}

fn demo_signed_odd<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        if i.odd() {
            println!("{} is odd", i);
        } else {
            println!("{} is not odd", i);
        }
    }
}

fn benchmark_unsigned_odd<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.odd()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|u| no_out!(u.odd())))],
    );
}

fn benchmark_signed_odd<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.odd()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|i| no_out!(i.odd())))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_odd::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_odd::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_odd::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_odd::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_odd, benchmark_u8_odd);
unsigned!(u16, demo_u16_odd, benchmark_u16_odd);
unsigned!(u32, demo_u32_odd, benchmark_u32_odd);
unsigned!(u64, demo_u64_odd, benchmark_u64_odd);
unsigned!(usize, demo_usize_odd, benchmark_usize_odd);

signed!(i8, demo_i8_odd, benchmark_i8_odd);
signed!(i16, demo_i16_odd, benchmark_i16_odd);
signed!(i32, demo_i32_odd, benchmark_i32_odd);
signed!(i64, demo_i64_odd, benchmark_i64_odd);
signed!(isize, demo_isize_odd, benchmark_isize_odd);
