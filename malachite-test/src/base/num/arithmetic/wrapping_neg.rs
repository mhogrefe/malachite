use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_wrapping_neg_assign);
    register_demo!(registry, demo_u16_wrapping_neg_assign);
    register_demo!(registry, demo_u32_wrapping_neg_assign);
    register_demo!(registry, demo_u64_wrapping_neg_assign);
    register_demo!(registry, demo_usize_wrapping_neg_assign);
    register_demo!(registry, demo_i8_wrapping_neg_assign);
    register_demo!(registry, demo_i16_wrapping_neg_assign);
    register_demo!(registry, demo_i32_wrapping_neg_assign);
    register_demo!(registry, demo_i64_wrapping_neg_assign);
    register_demo!(registry, demo_isize_wrapping_neg_assign);

    register_bench!(registry, None, benchmark_u8_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_u16_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_u32_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_u64_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_usize_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_i8_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_i16_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_i32_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_i64_wrapping_neg_assign);
    register_bench!(registry, None, benchmark_isize_wrapping_neg_assign);
}

fn demo_unsigned_wrapping_neg_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for mut u in unsigneds::<T>(gm).take(limit) {
        let old_u = u;
        u.wrapping_neg_assign();
        println!("u := {}; u.wrapping_neg_assign(); u = {}", old_u, u);
    }
}

fn demo_signed_wrapping_neg_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds::<T>(gm).take(limit) {
        let old_i = i;
        i.wrapping_neg_assign();
        println!("i := {}; i.wrapping_neg_assign(); i = {}", old_i, i);
    }
}

fn benchmark_unsigned_wrapping_neg_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_neg_assign()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|mut u| u.wrapping_neg_assign()))],
    );
}

fn benchmark_signed_wrapping_neg_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.wrapping_neg_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|mut i| i.wrapping_neg_assign()))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_wrapping_neg_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_wrapping_neg_assign::<$t>(gm, limit, file_name);
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
            demo_signed_wrapping_neg_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_wrapping_neg_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_wrapping_neg_assign,
    benchmark_u8_wrapping_neg_assign
);
unsigned!(
    u16,
    demo_u16_wrapping_neg_assign,
    benchmark_u16_wrapping_neg_assign
);
unsigned!(
    u32,
    demo_u32_wrapping_neg_assign,
    benchmark_u32_wrapping_neg_assign
);
unsigned!(
    u64,
    demo_u64_wrapping_neg_assign,
    benchmark_u64_wrapping_neg_assign
);
unsigned!(
    usize,
    demo_usize_wrapping_neg_assign,
    benchmark_usize_wrapping_neg_assign
);

signed!(
    i8,
    demo_i8_wrapping_neg_assign,
    benchmark_i8_wrapping_neg_assign
);
signed!(
    i16,
    demo_i16_wrapping_neg_assign,
    benchmark_i16_wrapping_neg_assign
);
signed!(
    i32,
    demo_i32_wrapping_neg_assign,
    benchmark_i32_wrapping_neg_assign
);
signed!(
    i64,
    demo_i64_wrapping_neg_assign,
    benchmark_i64_wrapping_neg_assign
);
signed!(
    isize,
    demo_isize_wrapping_neg_assign,
    benchmark_isize_wrapping_neg_assign
);
