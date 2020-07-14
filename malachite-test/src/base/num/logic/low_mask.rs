use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::small_u64s_var_4;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_low_mask);
    register_ns_demo!(registry, demo_u16_low_mask);
    register_ns_demo!(registry, demo_u32_low_mask);
    register_ns_demo!(registry, demo_u64_low_mask);
    register_ns_demo!(registry, demo_u128_low_mask);
    register_ns_demo!(registry, demo_usize_low_mask);
    register_ns_demo!(registry, demo_i8_low_mask);
    register_ns_demo!(registry, demo_i16_low_mask);
    register_ns_demo!(registry, demo_i32_low_mask);
    register_ns_demo!(registry, demo_i64_low_mask);
    register_ns_demo!(registry, demo_i128_low_mask);
    register_ns_demo!(registry, demo_isize_low_mask);

    register_ns_bench!(registry, None, benchmark_u8_low_mask);
    register_ns_bench!(registry, None, benchmark_u16_low_mask);
    register_ns_bench!(registry, None, benchmark_u32_low_mask);
    register_ns_bench!(registry, None, benchmark_u64_low_mask);
    register_ns_bench!(registry, None, benchmark_u128_low_mask);
    register_ns_bench!(registry, None, benchmark_usize_low_mask);
    register_ns_bench!(registry, None, benchmark_i8_low_mask);
    register_ns_bench!(registry, None, benchmark_i16_low_mask);
    register_ns_bench!(registry, None, benchmark_i32_low_mask);
    register_ns_bench!(registry, None, benchmark_i64_low_mask);
    register_ns_bench!(registry, None, benchmark_i128_low_mask);
    register_ns_bench!(registry, None, benchmark_isize_low_mask);
}

fn demo_unsigned_low_mask<T: PrimitiveUnsigned>(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_u64s_var_4::<T>(gm).take(limit) {
        println!("{}::low_mask({}) = {}", T::NAME, bits, T::low_mask(bits));
    }
}

fn demo_signed_low_mask<T: PrimitiveSigned>(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_u64s_var_4::<T>(gm).take(limit) {
        println!("{}::low_mask({}) = {}", T::NAME, bits, T::low_mask(bits));
    }
}

fn benchmark_unsigned_low_mask<T: PrimitiveUnsigned>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.low_mask(u64)", T::NAME),
        BenchmarkType::Single,
        small_u64s_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [("malachite", &mut (|bits| no_out!(T::low_mask(bits))))],
    );
}

fn benchmark_signed_low_mask<T: PrimitiveSigned>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.low_mask(u64)", T::NAME),
        BenchmarkType::Single,
        small_u64s_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [("malachite", &mut (|bits| no_out!(T::low_mask(bits))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_unsigned_low_mask::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_low_mask::<$t>(gm, limit, file_name);
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
            demo_signed_low_mask::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_low_mask::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_low_mask, benchmark_u8_low_mask);
unsigned!(u16, demo_u16_low_mask, benchmark_u16_low_mask);
unsigned!(u32, demo_u32_low_mask, benchmark_u32_low_mask);
unsigned!(u64, demo_u64_low_mask, benchmark_u64_low_mask);
unsigned!(u128, demo_u128_low_mask, benchmark_u128_low_mask);
unsigned!(usize, demo_usize_low_mask, benchmark_usize_low_mask);

signed!(i8, demo_i8_low_mask, benchmark_i8_low_mask);
signed!(i16, demo_i16_low_mask, benchmark_i16_low_mask);
signed!(i32, demo_i32_low_mask, benchmark_i32_low_mask);
signed!(i64, demo_i64_low_mask, benchmark_i64_low_mask);
signed!(i128, demo_i128_low_mask, benchmark_i128_low_mask);
signed!(isize, demo_isize_low_mask, benchmark_isize_low_mask);
