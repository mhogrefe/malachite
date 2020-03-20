use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::signeds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i8_saturating_abs);
    register_demo!(registry, demo_i16_saturating_abs);
    register_demo!(registry, demo_i32_saturating_abs);
    register_demo!(registry, demo_i64_saturating_abs);
    register_demo!(registry, demo_isize_saturating_abs);
    register_demo!(registry, demo_i8_saturating_abs_assign);
    register_demo!(registry, demo_i16_saturating_abs_assign);
    register_demo!(registry, demo_i32_saturating_abs_assign);
    register_demo!(registry, demo_i64_saturating_abs_assign);
    register_demo!(registry, demo_isize_saturating_abs_assign);
    register_bench!(registry, None, benchmark_i8_saturating_abs);
    register_bench!(registry, None, benchmark_i16_saturating_abs);
    register_bench!(registry, None, benchmark_i32_saturating_abs);
    register_bench!(registry, None, benchmark_i64_saturating_abs);
    register_bench!(registry, None, benchmark_isize_saturating_abs);
    register_bench!(registry, None, benchmark_i8_saturating_abs_assign);
    register_bench!(registry, None, benchmark_i16_saturating_abs_assign);
    register_bench!(registry, None, benchmark_i32_saturating_abs_assign);
    register_bench!(registry, None, benchmark_i64_saturating_abs_assign);
    register_bench!(registry, None, benchmark_isize_saturating_abs_assign);
}

fn demo_saturating_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("{}.saturating_abs() = {}", i, i.saturating_abs());
    }
}

fn demo_saturating_abs_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds::<T>(gm).take(limit) {
        let old_i = i;
        i.saturating_abs_assign();
        println!("i := {}; i.saturating_abs_assign(); i = {}", old_i, i);
    }
}

fn benchmark_saturating_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.saturating_abs()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|i| no_out!(i.saturating_abs())))],
    );
}

fn benchmark_saturating_abs_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.saturating_abs_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|mut i| i.saturating_abs_assign()))],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $demo_saturating_abs_name:ident,
        $demo_saturating_abs_assign_name:ident,
        $bench_saturating_abs_name:ident,
        $bench_saturating_abs_assign_name:ident
    ) => {
        fn $demo_saturating_abs_name(gm: GenerationMode, limit: usize) {
            demo_saturating_abs::<$t>(gm, limit);
        }

        fn $demo_saturating_abs_assign_name(gm: GenerationMode, limit: usize) {
            demo_saturating_abs_assign::<$t>(gm, limit);
        }

        fn $bench_saturating_abs_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_saturating_abs::<$t>(gm, limit, file_name);
        }

        fn $bench_saturating_abs_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_saturating_abs_assign::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    i8,
    demo_i8_saturating_abs,
    demo_i8_saturating_abs_assign,
    benchmark_i8_saturating_abs,
    benchmark_i8_saturating_abs_assign
);
demo_and_bench!(
    i16,
    demo_i16_saturating_abs,
    demo_i16_saturating_abs_assign,
    benchmark_i16_saturating_abs,
    benchmark_i16_saturating_abs_assign
);
demo_and_bench!(
    i32,
    demo_i32_saturating_abs,
    demo_i32_saturating_abs_assign,
    benchmark_i32_saturating_abs,
    benchmark_i32_saturating_abs_assign
);
demo_and_bench!(
    i64,
    demo_i64_saturating_abs,
    demo_i64_saturating_abs_assign,
    benchmark_i64_saturating_abs,
    benchmark_i64_saturating_abs_assign
);
demo_and_bench!(
    isize,
    demo_isize_saturating_abs,
    demo_isize_saturating_abs_assign,
    benchmark_isize_saturating_abs,
    benchmark_isize_saturating_abs_assign
);
