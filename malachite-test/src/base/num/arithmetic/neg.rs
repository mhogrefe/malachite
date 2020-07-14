use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::signeds_no_min;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i8_neg_assign);
    register_demo!(registry, demo_i16_neg_assign);
    register_demo!(registry, demo_i32_neg_assign);
    register_demo!(registry, demo_i64_neg_assign);
    register_demo!(registry, demo_isize_neg_assign);
    register_bench!(registry, None, benchmark_i8_neg_assign);
    register_bench!(registry, None, benchmark_i16_neg_assign);
    register_bench!(registry, None, benchmark_i32_neg_assign);
    register_bench!(registry, None, benchmark_i64_neg_assign);
    register_bench!(registry, None, benchmark_isize_neg_assign);
}

fn demo_neg_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds_no_min::<T>(gm).take(limit) {
        let old_i = i;
        i.neg_assign();
        println!("i := {}; i.neg_assign(); i = {}", old_i, i);
    }
}

fn benchmark_neg_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.neg_assign()", T::NAME),
        BenchmarkType::Single,
        signeds_no_min::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|mut i| i.neg_assign()))],
    );
}

macro_rules! demo_and_bench {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_neg_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_neg_assign::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(i8, demo_i8_neg_assign, benchmark_i8_neg_assign);
demo_and_bench!(i16, demo_i16_neg_assign, benchmark_i16_neg_assign);
demo_and_bench!(i32, demo_i32_neg_assign, benchmark_i32_neg_assign);
demo_and_bench!(i64, demo_i64_neg_assign, benchmark_i64_neg_assign);
demo_and_bench!(isize, demo_isize_neg_assign, benchmark_isize_neg_assign);
