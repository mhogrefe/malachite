use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::common::TRIPLE_SIGNIFICANT_BITS_LABEL;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, signeds, triples_of_signeds, triples_of_unsigneds,
    unsigneds,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_max_1);
    register_demo!(registry, demo_u16_max_1);
    register_demo!(registry, demo_u32_max_1);
    register_demo!(registry, demo_u64_max_1);
    register_demo!(registry, demo_usize_max_1);
    register_demo!(registry, demo_i8_max_1);
    register_demo!(registry, demo_i16_max_1);
    register_demo!(registry, demo_i32_max_1);
    register_demo!(registry, demo_i64_max_1);
    register_demo!(registry, demo_isize_max_1);

    register_demo!(registry, demo_u8_min_1);
    register_demo!(registry, demo_u16_min_1);
    register_demo!(registry, demo_u32_min_1);
    register_demo!(registry, demo_u64_min_1);
    register_demo!(registry, demo_usize_min_1);
    register_demo!(registry, demo_i8_min_1);
    register_demo!(registry, demo_i16_min_1);
    register_demo!(registry, demo_i32_min_1);
    register_demo!(registry, demo_i64_min_1);
    register_demo!(registry, demo_isize_min_1);

    register_demo!(registry, demo_u8_max_2);
    register_demo!(registry, demo_u16_max_2);
    register_demo!(registry, demo_u32_max_2);
    register_demo!(registry, demo_u64_max_2);
    register_demo!(registry, demo_usize_max_2);
    register_demo!(registry, demo_i8_max_2);
    register_demo!(registry, demo_i16_max_2);
    register_demo!(registry, demo_i32_max_2);
    register_demo!(registry, demo_i64_max_2);
    register_demo!(registry, demo_isize_max_2);

    register_demo!(registry, demo_u8_min_2);
    register_demo!(registry, demo_u16_min_2);
    register_demo!(registry, demo_u32_min_2);
    register_demo!(registry, demo_u64_min_2);
    register_demo!(registry, demo_usize_min_2);
    register_demo!(registry, demo_i8_min_2);
    register_demo!(registry, demo_i16_min_2);
    register_demo!(registry, demo_i32_min_2);
    register_demo!(registry, demo_i64_min_2);
    register_demo!(registry, demo_isize_min_2);

    register_demo!(registry, demo_u8_max_3);
    register_demo!(registry, demo_u16_max_3);
    register_demo!(registry, demo_u32_max_3);
    register_demo!(registry, demo_u64_max_3);
    register_demo!(registry, demo_usize_max_3);
    register_demo!(registry, demo_i8_max_3);
    register_demo!(registry, demo_i16_max_3);
    register_demo!(registry, demo_i32_max_3);
    register_demo!(registry, demo_i64_max_3);
    register_demo!(registry, demo_isize_max_3);

    register_demo!(registry, demo_u8_min_3);
    register_demo!(registry, demo_u16_min_3);
    register_demo!(registry, demo_u32_min_3);
    register_demo!(registry, demo_u64_min_3);
    register_demo!(registry, demo_usize_min_3);
    register_demo!(registry, demo_i8_min_3);
    register_demo!(registry, demo_i16_min_3);
    register_demo!(registry, demo_i32_min_3);
    register_demo!(registry, demo_i64_min_3);
    register_demo!(registry, demo_isize_min_3);

    register_bench!(registry, None, benchmark_u8_max_1);
    register_bench!(registry, None, benchmark_u16_max_1);
    register_bench!(registry, None, benchmark_u32_max_1);
    register_bench!(registry, None, benchmark_u64_max_1);
    register_bench!(registry, None, benchmark_usize_max_1);
    register_bench!(registry, None, benchmark_i8_max_1);
    register_bench!(registry, None, benchmark_i16_max_1);
    register_bench!(registry, None, benchmark_i32_max_1);
    register_bench!(registry, None, benchmark_i64_max_1);
    register_bench!(registry, None, benchmark_isize_max_1);

    register_bench!(registry, None, benchmark_u8_min_1);
    register_bench!(registry, None, benchmark_u16_min_1);
    register_bench!(registry, None, benchmark_u32_min_1);
    register_bench!(registry, None, benchmark_u64_min_1);
    register_bench!(registry, None, benchmark_usize_min_1);
    register_bench!(registry, None, benchmark_i8_min_1);
    register_bench!(registry, None, benchmark_i16_min_1);
    register_bench!(registry, None, benchmark_i32_min_1);
    register_bench!(registry, None, benchmark_i64_min_1);
    register_bench!(registry, None, benchmark_isize_min_1);

    register_bench!(registry, None, benchmark_u8_max_2);
    register_bench!(registry, None, benchmark_u16_max_2);
    register_bench!(registry, None, benchmark_u32_max_2);
    register_bench!(registry, None, benchmark_u64_max_2);
    register_bench!(registry, None, benchmark_usize_max_2);
    register_bench!(registry, None, benchmark_i8_max_2);
    register_bench!(registry, None, benchmark_i16_max_2);
    register_bench!(registry, None, benchmark_i32_max_2);
    register_bench!(registry, None, benchmark_i64_max_2);
    register_bench!(registry, None, benchmark_isize_max_2);

    register_bench!(registry, None, benchmark_u8_min_2);
    register_bench!(registry, None, benchmark_u16_min_2);
    register_bench!(registry, None, benchmark_u32_min_2);
    register_bench!(registry, None, benchmark_u64_min_2);
    register_bench!(registry, None, benchmark_usize_min_2);
    register_bench!(registry, None, benchmark_i8_min_2);
    register_bench!(registry, None, benchmark_i16_min_2);
    register_bench!(registry, None, benchmark_i32_min_2);
    register_bench!(registry, None, benchmark_i64_min_2);
    register_bench!(registry, None, benchmark_isize_min_2);

    register_bench!(registry, None, benchmark_u8_max_3);
    register_bench!(registry, None, benchmark_u16_max_3);
    register_bench!(registry, None, benchmark_u32_max_3);
    register_bench!(registry, None, benchmark_u64_max_3);
    register_bench!(registry, None, benchmark_usize_max_3);
    register_bench!(registry, None, benchmark_i8_max_3);
    register_bench!(registry, None, benchmark_i16_max_3);
    register_bench!(registry, None, benchmark_i32_max_3);
    register_bench!(registry, None, benchmark_i64_max_3);
    register_bench!(registry, None, benchmark_isize_max_3);

    register_bench!(registry, None, benchmark_u8_min_3);
    register_bench!(registry, None, benchmark_u16_min_3);
    register_bench!(registry, None, benchmark_u32_min_3);
    register_bench!(registry, None, benchmark_u64_min_3);
    register_bench!(registry, None, benchmark_usize_min_3);
    register_bench!(registry, None, benchmark_i8_min_3);
    register_bench!(registry, None, benchmark_i16_min_3);
    register_bench!(registry, None, benchmark_i32_min_3);
    register_bench!(registry, None, benchmark_i64_min_3);
    register_bench!(registry, None, benchmark_isize_min_3);
}

triple_significant_bits_fn!(u8, bucketing_function_u8);
triple_significant_bits_fn!(u16, bucketing_function_u16);
triple_significant_bits_fn!(u32, bucketing_function_u32);
triple_significant_bits_fn!(u64, bucketing_function_u64);
triple_significant_bits_fn!(usize, bucketing_function_usize);
triple_significant_bits_fn!(i8, bucketing_function_i8);
triple_significant_bits_fn!(i16, bucketing_function_i16);
triple_significant_bits_fn!(i32, bucketing_function_i32);
triple_significant_bits_fn!(i64, bucketing_function_i64);
triple_significant_bits_fn!(isize, bucketing_function_isize);

fn demo_unsigned_max_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for x in unsigneds::<T>(gm).take(limit) {
        println!("max!({}) = {}", x, max!(x));
    }
}

fn demo_signed_max_1<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for x in signeds::<T>(gm).take(limit) {
        println!("max!({}) = {}", x, max!(x));
    }
}

fn demo_unsigned_min_1<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for x in unsigneds::<T>(gm).take(limit) {
        println!("min!({}) = {}", x, min!(x));
    }
}

fn demo_signed_min_1<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for x in signeds::<T>(gm).take(limit) {
        println!("min!({}) = {}", x, min!(x));
    }
}

fn demo_unsigned_max_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("max!({}, {}) = {}", x, y, max!(x, y));
    }
}

fn demo_signed_max_2<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("max!({}, {}) = {}", x, y, max!(x, y));
    }
}

fn demo_unsigned_min_2<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("min!({}, {}) = {}", x, y, min!(x, y));
    }
}

fn demo_signed_min_2<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("min!({}, {}) = {}", x, y, min!(x, y));
    }
}

fn demo_unsigned_max_3<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, z) in triples_of_unsigneds::<T>(gm).take(limit) {
        println!("max!({}, {}, {}) = {}", x, y, z, max!(x, y, z));
    }
}

fn demo_signed_max_3<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y, z) in triples_of_signeds::<T>(gm).take(limit) {
        println!("max!({}, {}, {}) = {}", x, y, z, max!(x, y, z));
    }
}

fn demo_unsigned_min_3<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, z) in triples_of_unsigneds::<T>(gm).take(limit) {
        println!("min!({}, {}, {}) = {}", x, y, z, min!(x, y, z));
    }
}

fn demo_signed_min_3<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y, z) in triples_of_signeds::<T>(gm).take(limit) {
        println!("min!({}, {}, {}) = {}", x, y, z, min!(x, y, z));
    }
}

fn benchmark_unsigned_max_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("max!({})", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(max!(x))))],
    );
}

fn benchmark_signed_max_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("max!({})", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(max!(x))))],
    );
}

fn benchmark_unsigned_min_1<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("min!({})", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(min!(x))))],
    );
}

fn benchmark_signed_min_1<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("min!({})", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(min!(x))))],
    );
}

fn benchmark_unsigned_max_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("max_2({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(max!(x, y))))],
    );
}

fn benchmark_signed_max_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("max_2({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(max!(x, y))))],
    );
}

fn benchmark_unsigned_min_2<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("min!({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(min!(x, y))))],
    );
}

fn benchmark_signed_min_2<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("min!({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(min!(x, y))))],
    );
}

fn benchmark_unsigned_max_3<'a, T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
    bucketing_function: &'a dyn Fn(&(T, T, T)) -> usize,
) {
    m_run_benchmark(
        &format!("max!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [("malachite", &mut (|(x, y, z)| no_out!(max!(x, y, z))))],
    );
}

fn benchmark_signed_max_3<'a, T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
    bucketing_function: &'a dyn Fn(&(T, T, T)) -> usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("max!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [("malachite", &mut (|(x, y, z)| no_out!(max!(x, y, z))))],
    );
}

fn benchmark_unsigned_min_3<'a, T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
    bucketing_function: &'a dyn Fn(&(T, T, T)) -> usize,
) {
    m_run_benchmark(
        &format!("min!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [("malachite", &mut (|(x, y, z)| no_out!(min!(x, y, z))))],
    );
}

fn benchmark_signed_min_3<'a, T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
    bucketing_function: &'a dyn Fn(&(T, T, T)) -> usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("min!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [("malachite", &mut (|(x, y, z)| no_out!(min!(x, y, z))))],
    );
}

macro_rules! unsigned_max_1 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_max_1::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_max_1::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed_max_1 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_max_1::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_max_1::<$t>(gm, limit, file_name);
        }
    };
}

unsigned_max_1!(u8, demo_u8_max_1, benchmark_u8_max_1);
unsigned_max_1!(u16, demo_u16_max_1, benchmark_u16_max_1);
unsigned_max_1!(u32, demo_u32_max_1, benchmark_u32_max_1);
unsigned_max_1!(u64, demo_u64_max_1, benchmark_u64_max_1);
unsigned_max_1!(usize, demo_usize_max_1, benchmark_usize_max_1);

signed_max_1!(i8, demo_i8_max_1, benchmark_i8_max_1);
signed_max_1!(i16, demo_i16_max_1, benchmark_i16_max_1);
signed_max_1!(i32, demo_i32_max_1, benchmark_i32_max_1);
signed_max_1!(i64, demo_i64_max_1, benchmark_i64_max_1);
signed_max_1!(isize, demo_isize_max_1, benchmark_isize_max_1);

macro_rules! unsigned_min_1 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_min_1::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_min_1::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed_min_1 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_min_1::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_min_1::<$t>(gm, limit, file_name);
        }
    };
}

unsigned_min_1!(u8, demo_u8_min_1, benchmark_u8_min_1);
unsigned_min_1!(u16, demo_u16_min_1, benchmark_u16_min_1);
unsigned_min_1!(u32, demo_u32_min_1, benchmark_u32_min_1);
unsigned_min_1!(u64, demo_u64_min_1, benchmark_u64_min_1);
unsigned_min_1!(usize, demo_usize_min_1, benchmark_usize_min_1);

signed_min_1!(i8, demo_i8_min_1, benchmark_i8_min_1);
signed_min_1!(i16, demo_i16_min_1, benchmark_i16_min_1);
signed_min_1!(i32, demo_i32_min_1, benchmark_i32_min_1);
signed_min_1!(i64, demo_i64_min_1, benchmark_i64_min_1);
signed_min_1!(isize, demo_isize_min_1, benchmark_isize_min_1);

macro_rules! unsigned_max_2 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_max_2::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_max_2::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed_max_2 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_max_2::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_max_2::<$t>(gm, limit, file_name);
        }
    };
}

unsigned_max_2!(u8, demo_u8_max_2, benchmark_u8_max_2);
unsigned_max_2!(u16, demo_u16_max_2, benchmark_u16_max_2);
unsigned_max_2!(u32, demo_u32_max_2, benchmark_u32_max_2);
unsigned_max_2!(u64, demo_u64_max_2, benchmark_u64_max_2);
unsigned_max_2!(usize, demo_usize_max_2, benchmark_usize_max_2);

signed_max_2!(i8, demo_i8_max_2, benchmark_i8_max_2);
signed_max_2!(i16, demo_i16_max_2, benchmark_i16_max_2);
signed_max_2!(i32, demo_i32_max_2, benchmark_i32_max_2);
signed_max_2!(i64, demo_i64_max_2, benchmark_i64_max_2);
signed_max_2!(isize, demo_isize_max_2, benchmark_isize_max_2);

macro_rules! unsigned_min_2 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_min_2::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_min_2::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed_min_2 {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_min_2::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_min_2::<$t>(gm, limit, file_name);
        }
    };
}

unsigned_min_2!(u8, demo_u8_min_2, benchmark_u8_min_2);
unsigned_min_2!(u16, demo_u16_min_2, benchmark_u16_min_2);
unsigned_min_2!(u32, demo_u32_min_2, benchmark_u32_min_2);
unsigned_min_2!(u64, demo_u64_min_2, benchmark_u64_min_2);
unsigned_min_2!(usize, demo_usize_min_2, benchmark_usize_min_2);

signed_min_2!(i8, demo_i8_min_2, benchmark_i8_min_2);
signed_min_2!(i16, demo_i16_min_2, benchmark_i16_min_2);
signed_min_2!(i32, demo_i32_min_2, benchmark_i32_min_2);
signed_min_2!(i64, demo_i64_min_2, benchmark_i64_min_2);
signed_min_2!(isize, demo_isize_min_2, benchmark_isize_min_2);

macro_rules! unsigned_max_3 {
    (
        $t:ident,
        $bucketing_function:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_max_3::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_max_3::<$t>(gm, limit, file_name, &$bucketing_function);
        }
    };
}

macro_rules! signed_max_3 {
    (
        $t:ident,
        $bucketing_function:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_max_3::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_max_3::<$t>(gm, limit, file_name, &$bucketing_function);
        }
    };
}

unsigned_max_3!(u8, bucketing_function_u8, demo_u8_max_3, benchmark_u8_max_3);
unsigned_max_3!(
    u16,
    bucketing_function_u16,
    demo_u16_max_3,
    benchmark_u16_max_3
);
unsigned_max_3!(
    u32,
    bucketing_function_u32,
    demo_u32_max_3,
    benchmark_u32_max_3
);
unsigned_max_3!(
    u64,
    bucketing_function_u64,
    demo_u64_max_3,
    benchmark_u64_max_3
);
unsigned_max_3!(
    usize,
    bucketing_function_usize,
    demo_usize_max_3,
    benchmark_usize_max_3
);

signed_max_3!(i8, bucketing_function_i8, demo_i8_max_3, benchmark_i8_max_3);
signed_max_3!(
    i16,
    bucketing_function_i16,
    demo_i16_max_3,
    benchmark_i16_max_3
);
signed_max_3!(
    i32,
    bucketing_function_i32,
    demo_i32_max_3,
    benchmark_i32_max_3
);
signed_max_3!(
    i64,
    bucketing_function_i64,
    demo_i64_max_3,
    benchmark_i64_max_3
);
signed_max_3!(
    isize,
    bucketing_function_isize,
    demo_isize_max_3,
    benchmark_isize_max_3
);

macro_rules! unsigned_min_3 {
    (
        $t:ident,
        $bucketing_function:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_min_3::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_min_3::<$t>(gm, limit, file_name, &$bucketing_function);
        }
    };
}

macro_rules! signed_min_3 {
    (
        $t:ident,
        $bucketing_function:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_min_3::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_min_3::<$t>(gm, limit, file_name, &$bucketing_function);
        }
    };
}

unsigned_min_3!(u8, bucketing_function_u8, demo_u8_min_3, benchmark_u8_min_3);
unsigned_min_3!(
    u16,
    bucketing_function_u16,
    demo_u16_min_3,
    benchmark_u16_min_3
);
unsigned_min_3!(
    u32,
    bucketing_function_u32,
    demo_u32_min_3,
    benchmark_u32_min_3
);
unsigned_min_3!(
    u64,
    bucketing_function_u64,
    demo_u64_min_3,
    benchmark_u64_min_3
);
unsigned_min_3!(
    usize,
    bucketing_function_usize,
    demo_usize_min_3,
    benchmark_usize_min_3
);

signed_min_3!(i8, bucketing_function_i8, demo_i8_min_3, benchmark_i8_min_3);
signed_min_3!(
    i16,
    bucketing_function_i16,
    demo_i16_min_3,
    benchmark_i16_min_3
);
signed_min_3!(
    i32,
    bucketing_function_i32,
    demo_i32_min_3,
    benchmark_i32_min_3
);
signed_min_3!(
    i64,
    bucketing_function_i64,
    demo_i64_min_3,
    benchmark_i64_min_3
);
signed_min_3!(
    isize,
    bucketing_function_isize,
    demo_isize_min_3,
    benchmark_isize_min_3
);
