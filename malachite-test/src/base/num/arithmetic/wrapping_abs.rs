use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::signeds;

//TODO add unsigned wrapping_abs
pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i8_wrapping_abs_assign);
    register_demo!(registry, demo_i16_wrapping_abs_assign);
    register_demo!(registry, demo_i32_wrapping_abs_assign);
    register_demo!(registry, demo_i64_wrapping_abs_assign);
    register_demo!(registry, demo_isize_wrapping_abs_assign);
    register_bench!(registry, None, benchmark_i8_wrapping_abs_assign);
    register_bench!(registry, None, benchmark_i16_wrapping_abs_assign);
    register_bench!(registry, None, benchmark_i32_wrapping_abs_assign);
    register_bench!(registry, None, benchmark_i64_wrapping_abs_assign);
    register_bench!(registry, None, benchmark_isize_wrapping_abs_assign);
}

fn demo_wrapping_abs_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds::<T>(gm).take(limit) {
        let old_i = i;
        i.wrapping_abs_assign();
        println!("i := {}; i.wrapping_abs_assign(); i = {}", old_i, i);
    }
}

fn benchmark_wrapping_abs_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.wrapping_abs_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|mut i| i.wrapping_abs_assign()))],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_wrapping_abs_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_wrapping_abs_assign::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    i8,
    demo_i8_wrapping_abs_assign,
    benchmark_i8_wrapping_abs_assign
);
demo_and_bench!(
    i16,
    demo_i16_wrapping_abs_assign,
    benchmark_i16_wrapping_abs_assign
);
demo_and_bench!(
    i32,
    demo_i32_wrapping_abs_assign,
    benchmark_i32_wrapping_abs_assign
);
demo_and_bench!(
    i64,
    demo_i64_wrapping_abs_assign,
    benchmark_i64_wrapping_abs_assign
);
demo_and_bench!(
    isize,
    demo_isize_wrapping_abs_assign,
    benchmark_isize_wrapping_abs_assign
);
