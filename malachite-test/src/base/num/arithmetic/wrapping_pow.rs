use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use rand::Rand;

use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::{pairs_of_small_signed_and_small_unsigned, pairs_of_small_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_wrapping_pow_assign);
    register_ns_demo!(registry, demo_u16_wrapping_pow_assign);
    register_ns_demo!(registry, demo_u32_wrapping_pow_assign);
    register_ns_demo!(registry, demo_u64_wrapping_pow_assign);
    register_ns_demo!(registry, demo_usize_wrapping_pow_assign);
    register_ns_demo!(registry, demo_i8_wrapping_pow_assign);
    register_ns_demo!(registry, demo_i16_wrapping_pow_assign);
    register_ns_demo!(registry, demo_i32_wrapping_pow_assign);
    register_ns_demo!(registry, demo_i64_wrapping_pow_assign);
    register_ns_demo!(registry, demo_isize_wrapping_pow_assign);

    register_ns_bench!(registry, None, benchmark_u8_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_u16_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_u32_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_u64_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_usize_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_i8_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_i16_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_i32_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_i64_wrapping_pow_assign);
    register_ns_bench!(registry, None, benchmark_isize_wrapping_pow_assign);
}

fn demo_unsigned_wrapping_pow_assign<T: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (mut x, y) in pairs_of_small_unsigneds::<T, u64>(gm).take(limit) {
        let old_x = x;
        x.wrapping_pow_assign(y);
        println!("x := {}; x.wrapping_pow_assign({}); x = {}", old_x, y, x);
    }
}

fn demo_signed_wrapping_pow_assign<T: PrimitiveSigned + Rand>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (mut x, y) in pairs_of_small_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        let old_x = x;
        x.wrapping_pow_assign(y);
        println!("x := {}; x.wrapping_pow_assign({}); x = {}", old_x, y, x);
    }
}

fn benchmark_unsigned_wrapping_pow_assign<T: PrimitiveUnsigned + Rand>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.wrapping_pow_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_small_unsigneds::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| x.wrapping_pow_assign(y)))],
    );
}

fn benchmark_signed_wrapping_pow_assign<T: PrimitiveSigned + Rand>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.wrapping_pow_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_small_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| x.wrapping_pow_assign(y)))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_unsigned_wrapping_pow_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_wrapping_pow_assign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_signed_wrapping_pow_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_wrapping_pow_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_wrapping_pow_assign,
    benchmark_u8_wrapping_pow_assign
);
unsigned!(
    u16,
    demo_u16_wrapping_pow_assign,
    benchmark_u16_wrapping_pow_assign
);
unsigned!(
    u32,
    demo_u32_wrapping_pow_assign,
    benchmark_u32_wrapping_pow_assign
);
unsigned!(
    u64,
    demo_u64_wrapping_pow_assign,
    benchmark_u64_wrapping_pow_assign
);
unsigned!(
    usize,
    demo_usize_wrapping_pow_assign,
    benchmark_usize_wrapping_pow_assign
);

signed!(
    i8,
    demo_i8_wrapping_pow_assign,
    benchmark_i8_wrapping_pow_assign
);
signed!(
    i16,
    demo_i16_wrapping_pow_assign,
    benchmark_i16_wrapping_pow_assign
);
signed!(
    i32,
    demo_i32_wrapping_pow_assign,
    benchmark_i32_wrapping_pow_assign
);
signed!(
    i64,
    demo_i64_wrapping_pow_assign,
    benchmark_i64_wrapping_pow_assign
);
signed!(
    isize,
    demo_isize_wrapping_pow_assign,
    benchmark_isize_wrapping_pow_assign
);
