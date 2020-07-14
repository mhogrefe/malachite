use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_saturating_square);
    register_demo!(registry, demo_u16_saturating_square);
    register_demo!(registry, demo_u32_saturating_square);
    register_demo!(registry, demo_u64_saturating_square);
    register_demo!(registry, demo_usize_saturating_square);
    register_demo!(registry, demo_i8_saturating_square);
    register_demo!(registry, demo_i16_saturating_square);
    register_demo!(registry, demo_i32_saturating_square);
    register_demo!(registry, demo_i64_saturating_square);
    register_demo!(registry, demo_isize_saturating_square);

    register_demo!(registry, demo_u8_saturating_square_assign);
    register_demo!(registry, demo_u16_saturating_square_assign);
    register_demo!(registry, demo_u32_saturating_square_assign);
    register_demo!(registry, demo_u64_saturating_square_assign);
    register_demo!(registry, demo_usize_saturating_square_assign);
    register_demo!(registry, demo_i8_saturating_square_assign);
    register_demo!(registry, demo_i16_saturating_square_assign);
    register_demo!(registry, demo_i32_saturating_square_assign);
    register_demo!(registry, demo_i64_saturating_square_assign);
    register_demo!(registry, demo_isize_saturating_square_assign);

    register_bench!(registry, None, benchmark_u8_saturating_square);
    register_bench!(registry, None, benchmark_u16_saturating_square);
    register_bench!(registry, None, benchmark_u32_saturating_square);
    register_bench!(registry, None, benchmark_u64_saturating_square);
    register_bench!(registry, None, benchmark_usize_saturating_square);
    register_bench!(registry, None, benchmark_i8_saturating_square);
    register_bench!(registry, None, benchmark_i16_saturating_square);
    register_bench!(registry, None, benchmark_i32_saturating_square);
    register_bench!(registry, None, benchmark_i64_saturating_square);
    register_bench!(registry, None, benchmark_isize_saturating_square);

    register_bench!(registry, None, benchmark_u8_saturating_square_assign);
    register_bench!(registry, None, benchmark_u16_saturating_square_assign);
    register_bench!(registry, None, benchmark_u32_saturating_square_assign);
    register_bench!(registry, None, benchmark_u64_saturating_square_assign);
    register_bench!(registry, None, benchmark_usize_saturating_square_assign);
    register_bench!(registry, None, benchmark_i8_saturating_square_assign);
    register_bench!(registry, None, benchmark_i16_saturating_square_assign);
    register_bench!(registry, None, benchmark_i32_saturating_square_assign);
    register_bench!(registry, None, benchmark_i64_saturating_square_assign);
    register_bench!(registry, None, benchmark_isize_saturating_square_assign);
}

fn demo_unsigned_saturating_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for x in unsigneds::<T>(gm).take(limit) {
        println!("{}.saturating_square() = {}", x, x.saturating_square());
    }
}

fn demo_unsigned_saturating_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for mut x in unsigneds::<T>(gm).take(limit) {
        let old_x = x;
        x.saturating_square_assign();
        println!("x := {}; x.saturating_square_assign(); x = {}", old_x, x);
    }
}

fn demo_signed_saturating_square<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for x in signeds::<T>(gm).take(limit) {
        println!("{}.saturating_square() = {}", x, x.saturating_square());
    }
}

fn demo_signed_saturating_square_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut x in signeds::<T>(gm).take(limit) {
        let old_x = x;
        x.saturating_square_assign();
        println!("x := {}; x.saturating_square_assign(); x = {}", old_x, x);
    }
}

fn benchmark_unsigned_saturating_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.saturating_square()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(x.saturating_square())))],
    );
}

fn benchmark_unsigned_saturating_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.saturating_square_assign()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|mut x| x.saturating_square_assign()))],
    );
}

fn benchmark_signed_saturating_square<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.saturating_square()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(x.saturating_square())))],
    );
}

fn benchmark_signed_saturating_square_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.saturating_square_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|mut x| x.saturating_square_assign()))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_saturating_square::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_saturating_square_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_saturating_square::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_saturating_square_assign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_saturating_square::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_signed_saturating_square_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_saturating_square::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_saturating_square_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_saturating_square,
    demo_u8_saturating_square_assign,
    benchmark_u8_saturating_square,
    benchmark_u8_saturating_square_assign
);
unsigned!(
    u16,
    demo_u16_saturating_square,
    demo_u16_saturating_square_assign,
    benchmark_u16_saturating_square,
    benchmark_u16_saturating_square_assign
);
unsigned!(
    u32,
    demo_u32_saturating_square,
    demo_u32_saturating_square_assign,
    benchmark_u32_saturating_square,
    benchmark_u32_saturating_square_assign
);
unsigned!(
    u64,
    demo_u64_saturating_square,
    demo_u64_saturating_square_assign,
    benchmark_u64_saturating_square,
    benchmark_u64_saturating_square_assign
);
unsigned!(
    usize,
    demo_usize_saturating_square,
    demo_usize_saturating_square_assign,
    benchmark_usize_saturating_square,
    benchmark_usize_saturating_square_assign
);

signed!(
    i8,
    demo_i8_saturating_square,
    demo_i8_saturating_square_assign,
    benchmark_i8_saturating_square,
    benchmark_i8_saturating_square_assign
);
signed!(
    i16,
    demo_i16_saturating_square,
    demo_i16_saturating_square_assign,
    benchmark_i16_saturating_square,
    benchmark_i16_saturating_square_assign
);
signed!(
    i32,
    demo_i32_saturating_square,
    demo_i32_saturating_square_assign,
    benchmark_i32_saturating_square,
    benchmark_i32_saturating_square_assign
);
signed!(
    i64,
    demo_i64_saturating_square,
    demo_i64_saturating_square_assign,
    benchmark_i64_saturating_square,
    benchmark_i64_saturating_square_assign
);
signed!(
    isize,
    demo_isize_saturating_square,
    demo_isize_saturating_square_assign,
    benchmark_isize_saturating_square,
    benchmark_isize_saturating_square_assign
);
