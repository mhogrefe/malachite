use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::{small_u64s_var_2, small_u64s_var_3};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_power_of_two);
    register_ns_demo!(registry, demo_u16_power_of_two);
    register_ns_demo!(registry, demo_u32_power_of_two);
    register_ns_demo!(registry, demo_u64_power_of_two);
    register_ns_demo!(registry, demo_u128_power_of_two);
    register_ns_demo!(registry, demo_usize_power_of_two);
    register_ns_demo!(registry, demo_i8_power_of_two);
    register_ns_demo!(registry, demo_i16_power_of_two);
    register_ns_demo!(registry, demo_i32_power_of_two);
    register_ns_demo!(registry, demo_i64_power_of_two);
    register_ns_demo!(registry, demo_i128_power_of_two);
    register_ns_demo!(registry, demo_isize_power_of_two);

    register_ns_bench!(registry, None, benchmark_u8_power_of_two);
    register_ns_bench!(registry, None, benchmark_u16_power_of_two);
    register_ns_bench!(registry, None, benchmark_u32_power_of_two);
    register_ns_bench!(registry, None, benchmark_u64_power_of_two);
    register_ns_bench!(registry, None, benchmark_u128_power_of_two);
    register_ns_bench!(registry, None, benchmark_usize_power_of_two);
    register_ns_bench!(registry, None, benchmark_i8_power_of_two);
    register_ns_bench!(registry, None, benchmark_i16_power_of_two);
    register_ns_bench!(registry, None, benchmark_i32_power_of_two);
    register_ns_bench!(registry, None, benchmark_i64_power_of_two);
    register_ns_bench!(registry, None, benchmark_i128_power_of_two);
    register_ns_bench!(registry, None, benchmark_isize_power_of_two);
}

fn demo_unsigned_power_of_two<T: PrimitiveUnsigned>(gm: NoSpecialGenerationMode, limit: usize) {
    for pow in small_u64s_var_2::<T>(gm).take(limit) {
        println!("2^{} = {}", pow, T::power_of_two(pow));
    }
}

fn demo_signed_power_of_two<T: PrimitiveSigned>(gm: NoSpecialGenerationMode, limit: usize) {
    for pow in small_u64s_var_3::<T>(gm).take(limit) {
        println!("2^{} = {}", pow, T::power_of_two(pow));
    }
}

fn benchmark_unsigned_power_of_two<T: PrimitiveUnsigned>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.power_of_two(u64)", T::NAME),
        BenchmarkType::Single,
        small_u64s_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&pow| usize::exact_from(pow)),
        "pow",
        &mut [("malachite", &mut (|pow| no_out!(T::power_of_two(pow))))],
    );
}

fn benchmark_signed_power_of_two<T: PrimitiveSigned>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.power_of_two(u64)", T::NAME),
        BenchmarkType::Single,
        small_u64s_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&pow| usize::exact_from(pow)),
        "pow",
        &mut [("malachite", &mut (|pow| no_out!(T::power_of_two(pow))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_unsigned_power_of_two::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_power_of_two::<$t>(gm, limit, file_name);
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
            demo_signed_power_of_two::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_power_of_two::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_power_of_two, benchmark_u8_power_of_two);
unsigned!(u16, demo_u16_power_of_two, benchmark_u16_power_of_two);
unsigned!(u32, demo_u32_power_of_two, benchmark_u32_power_of_two);
unsigned!(u64, demo_u64_power_of_two, benchmark_u64_power_of_two);
unsigned!(u128, demo_u128_power_of_two, benchmark_u128_power_of_two);
unsigned!(usize, demo_usize_power_of_two, benchmark_usize_power_of_two);

signed!(i8, demo_i8_power_of_two, benchmark_i8_power_of_two);
signed!(i16, demo_i16_power_of_two, benchmark_i16_power_of_two);
signed!(i32, demo_i32_power_of_two, benchmark_i32_power_of_two);
signed!(i64, demo_i64_power_of_two, benchmark_i64_power_of_two);
signed!(i128, demo_i128_power_of_two, benchmark_i128_power_of_two);
signed!(isize, demo_isize_power_of_two, benchmark_isize_power_of_two);
