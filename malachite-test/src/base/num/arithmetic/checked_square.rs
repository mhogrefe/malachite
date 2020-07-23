use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_checked_square);
    register_demo!(registry, demo_u16_checked_square);
    register_demo!(registry, demo_u32_checked_square);
    register_demo!(registry, demo_u64_checked_square);
    register_demo!(registry, demo_usize_checked_square);
    register_demo!(registry, demo_i8_checked_square);
    register_demo!(registry, demo_i16_checked_square);
    register_demo!(registry, demo_i32_checked_square);
    register_demo!(registry, demo_i64_checked_square);
    register_demo!(registry, demo_isize_checked_square);

    register_bench!(registry, None, benchmark_u8_checked_square);
    register_bench!(registry, None, benchmark_u16_checked_square);
    register_bench!(registry, None, benchmark_u32_checked_square);
    register_bench!(registry, None, benchmark_u64_checked_square);
    register_bench!(registry, None, benchmark_usize_checked_square);
    register_bench!(registry, None, benchmark_i8_checked_square);
    register_bench!(registry, None, benchmark_i16_checked_square);
    register_bench!(registry, None, benchmark_i32_checked_square);
    register_bench!(registry, None, benchmark_i64_checked_square);
    register_bench!(registry, None, benchmark_isize_checked_square);
}

fn demo_unsigned_checked_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for x in unsigneds::<T>(gm).take(limit) {
        println!("{}.checked_square() = {:?}", x, x.checked_square());
    }
}

fn demo_signed_checked_square<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for x in signeds::<T>(gm).take(limit) {
        println!("{}.checked_square() = {:?}", x, x.checked_square());
    }
}

fn benchmark_unsigned_checked_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_square()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(x.checked_square())))],
    );
}

fn benchmark_signed_checked_square<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.checked_square()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|x| no_out!(x.checked_square())))],
    );
}
macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_checked_square::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_checked_square::<$t>(gm, limit, file_name);
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
            demo_signed_checked_square::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_checked_square::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_checked_square, benchmark_u8_checked_square);
unsigned!(u16, demo_u16_checked_square, benchmark_u16_checked_square);
unsigned!(u32, demo_u32_checked_square, benchmark_u32_checked_square);
unsigned!(u64, demo_u64_checked_square, benchmark_u64_checked_square);
unsigned!(
    usize,
    demo_usize_checked_square,
    benchmark_usize_checked_square
);

signed!(i8, demo_i8_checked_square, benchmark_i8_checked_square);
signed!(i16, demo_i16_checked_square, benchmark_i16_checked_square);
signed!(i32, demo_i32_checked_square, benchmark_i32_checked_square);
signed!(i64, demo_i64_checked_square, benchmark_i64_checked_square);
signed!(
    isize,
    demo_isize_checked_square,
    benchmark_isize_checked_square
);
