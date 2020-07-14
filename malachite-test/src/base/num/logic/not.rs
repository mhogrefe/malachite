use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_not_assign);
    register_demo!(registry, demo_u16_not_assign);
    register_demo!(registry, demo_u32_not_assign);
    register_demo!(registry, demo_u64_not_assign);
    register_demo!(registry, demo_usize_not_assign);
    register_demo!(registry, demo_i8_not_assign);
    register_demo!(registry, demo_i16_not_assign);
    register_demo!(registry, demo_i32_not_assign);
    register_demo!(registry, demo_i64_not_assign);
    register_demo!(registry, demo_isize_not_assign);
    register_bench!(registry, None, benchmark_u8_not_assign);
    register_bench!(registry, None, benchmark_u16_not_assign);
    register_bench!(registry, None, benchmark_u32_not_assign);
    register_bench!(registry, None, benchmark_u64_not_assign);
    register_bench!(registry, None, benchmark_usize_not_assign);
    register_bench!(registry, None, benchmark_i8_not_assign);
    register_bench!(registry, None, benchmark_i16_not_assign);
    register_bench!(registry, None, benchmark_i32_not_assign);
    register_bench!(registry, None, benchmark_i64_not_assign);
    register_bench!(registry, None, benchmark_isize_not_assign);
}

fn demo_unsigned_not_assign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for mut u in unsigneds::<T>(gm).take(limit) {
        let old_u = u;
        u.not_assign();
        println!("u := {}; u.not_assign(); u = {}", old_u, u);
    }
}

fn demo_signed_not_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds::<T>(gm).take(limit) {
        let old_i = i;
        i.not_assign();
        println!("i := {}; i.not_assign(); i = {}", old_i, i);
    }
}

fn benchmark_unsigned_not_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.not_assign()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|mut u| u.not_assign()))],
    );
}

fn benchmark_signed_not_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.not_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|mut i| i.not_assign()))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_not_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_not_assign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_not_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_not_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_not_assign, benchmark_u8_not_assign);
unsigned!(u16, demo_u16_not_assign, benchmark_u16_not_assign);
unsigned!(u32, demo_u32_not_assign, benchmark_u32_not_assign);
unsigned!(u64, demo_u64_not_assign, benchmark_u64_not_assign);
unsigned!(usize, demo_usize_not_assign, benchmark_usize_not_assign);

signed!(i8, demo_i8_not_assign, benchmark_i8_not_assign);
signed!(i16, demo_i16_not_assign, benchmark_i16_not_assign);
signed!(i32, demo_i32_not_assign, benchmark_i32_not_assign);
signed!(i64, demo_i64_not_assign, benchmark_i64_not_assign);
signed!(isize, demo_isize_not_assign, benchmark_isize_not_assign);
