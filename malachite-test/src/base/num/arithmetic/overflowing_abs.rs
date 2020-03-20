use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::signeds;

//TODO add unsigned overflowing_abs
pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i8_overflowing_abs_assign);
    register_demo!(registry, demo_i16_overflowing_abs_assign);
    register_demo!(registry, demo_i32_overflowing_abs_assign);
    register_demo!(registry, demo_i64_overflowing_abs_assign);
    register_demo!(registry, demo_isize_overflowing_abs_assign);
    register_bench!(registry, None, benchmark_i8_overflowing_abs_assign);
    register_bench!(registry, None, benchmark_i16_overflowing_abs_assign);
    register_bench!(registry, None, benchmark_i32_overflowing_abs_assign);
    register_bench!(registry, None, benchmark_i64_overflowing_abs_assign);
    register_bench!(registry, None, benchmark_isize_overflowing_abs_assign);
}

fn demo_overflowing_abs_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds::<T>(gm).take(limit) {
        let old_i = i;
        let overflow = i.overflowing_neg_assign();
        println!(
            "i := {}; i.overflowing_neg_assign() = {}; i = {}",
            old_i, overflow, i
        );
    }
}

fn benchmark_overflowing_abs_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.overflowing_abs_assign()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::wrapping_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [(
            "malachite",
            &mut (|mut i| no_out!(i.overflowing_abs_assign())),
        )],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_overflowing_abs_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_overflowing_abs_assign::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    i8,
    demo_i8_overflowing_abs_assign,
    benchmark_i8_overflowing_abs_assign
);
demo_and_bench!(
    i16,
    demo_i16_overflowing_abs_assign,
    benchmark_i16_overflowing_abs_assign
);
demo_and_bench!(
    i32,
    demo_i32_overflowing_abs_assign,
    benchmark_i32_overflowing_abs_assign
);
demo_and_bench!(
    i64,
    demo_i64_overflowing_abs_assign,
    benchmark_i64_overflowing_abs_assign
);
demo_and_bench!(
    isize,
    demo_isize_overflowing_abs_assign,
    benchmark_isize_overflowing_abs_assign
);
