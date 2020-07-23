use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signed_and_nonzero_signed, pairs_of_unsigned_and_positive_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_wrapping_div_assign);
    register_demo!(registry, demo_u16_wrapping_div_assign);
    register_demo!(registry, demo_u32_wrapping_div_assign);
    register_demo!(registry, demo_u64_wrapping_div_assign);
    register_demo!(registry, demo_usize_wrapping_div_assign);
    register_demo!(registry, demo_i8_wrapping_div_assign);
    register_demo!(registry, demo_i16_wrapping_div_assign);
    register_demo!(registry, demo_i32_wrapping_div_assign);
    register_demo!(registry, demo_i64_wrapping_div_assign);
    register_demo!(registry, demo_isize_wrapping_div_assign);

    register_bench!(registry, None, benchmark_u8_wrapping_div_assign);
    register_bench!(registry, None, benchmark_u16_wrapping_div_assign);
    register_bench!(registry, None, benchmark_u32_wrapping_div_assign);
    register_bench!(registry, None, benchmark_u64_wrapping_div_assign);
    register_bench!(registry, None, benchmark_usize_wrapping_div_assign);
    register_bench!(registry, None, benchmark_i8_wrapping_div_assign);
    register_bench!(registry, None, benchmark_i16_wrapping_div_assign);
    register_bench!(registry, None, benchmark_i32_wrapping_div_assign);
    register_bench!(registry, None, benchmark_i64_wrapping_div_assign);
    register_bench!(registry, None, benchmark_isize_wrapping_div_assign);
}

fn demo_unsigned_wrapping_div_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        let old_x = x;
        x.wrapping_div_assign(y);
        println!("x := {}; x.wrapping_div_assign({}); x = {}", old_x, y, x);
    }
}

fn demo_signed_wrapping_div_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signed_and_nonzero_signed::<T, T>(gm).take(limit) {
        let old_x = x;
        x.wrapping_div_assign(y);
        println!("x := {}; x.wrapping_div_assign({}); x = {}", old_x, y, x);
    }
}

fn benchmark_unsigned_wrapping_div_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_div_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| x.wrapping_div_assign(y)))],
    );
}

fn benchmark_signed_wrapping_div_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.wrapping_div_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_nonzero_signed::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| x.wrapping_div_assign(y)))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_wrapping_div_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_wrapping_div_assign::<$t>(gm, limit, file_name);
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
            demo_signed_wrapping_div_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_wrapping_div_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_wrapping_div_assign,
    benchmark_u8_wrapping_div_assign
);
unsigned!(
    u16,
    demo_u16_wrapping_div_assign,
    benchmark_u16_wrapping_div_assign
);
unsigned!(
    u32,
    demo_u32_wrapping_div_assign,
    benchmark_u32_wrapping_div_assign
);
unsigned!(
    u64,
    demo_u64_wrapping_div_assign,
    benchmark_u64_wrapping_div_assign
);
unsigned!(
    usize,
    demo_usize_wrapping_div_assign,
    benchmark_usize_wrapping_div_assign
);

signed!(
    i8,
    demo_i8_wrapping_div_assign,
    benchmark_i8_wrapping_div_assign
);
signed!(
    i16,
    demo_i16_wrapping_div_assign,
    benchmark_i16_wrapping_div_assign
);
signed!(
    i32,
    demo_i32_wrapping_div_assign,
    benchmark_i32_wrapping_div_assign
);
signed!(
    i64,
    demo_i64_wrapping_div_assign,
    benchmark_i64_wrapping_div_assign
);
signed!(
    isize,
    demo_isize_wrapping_div_assign,
    benchmark_isize_wrapping_div_assign
);
