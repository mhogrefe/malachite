use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{signeds, signeds_no_max};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i8_abs_assign);
    register_demo!(registry, demo_i16_abs_assign);
    register_demo!(registry, demo_i32_abs_assign);
    register_demo!(registry, demo_i64_abs_assign);
    register_demo!(registry, demo_isize_abs_assign);
    register_demo!(registry, demo_i8_unsigned_abs);
    register_demo!(registry, demo_i16_unsigned_abs);
    register_demo!(registry, demo_i32_unsigned_abs);
    register_demo!(registry, demo_i64_unsigned_abs);
    register_demo!(registry, demo_isize_unsigned_abs);

    register_bench!(registry, None, benchmark_i8_abs_assign);
    register_bench!(registry, None, benchmark_i16_abs_assign);
    register_bench!(registry, None, benchmark_i32_abs_assign);
    register_bench!(registry, None, benchmark_i64_abs_assign);
    register_bench!(registry, None, benchmark_isize_abs_assign);
    register_bench!(registry, None, benchmark_i8_unsigned_abs);
    register_bench!(registry, None, benchmark_i16_unsigned_abs);
    register_bench!(registry, None, benchmark_i32_unsigned_abs);
    register_bench!(registry, None, benchmark_i64_unsigned_abs);
    register_bench!(registry, None, benchmark_isize_unsigned_abs);
}

fn demo_abs_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut i in signeds_no_max::<T>(gm).take(limit) {
        let old_i = i;
        i.abs_assign();
        println!("i := {}; i.abs_assign(); i = {}", old_i, i);
    }
}

fn demo_unsigned_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("{}.unsigned_abs() = {}", i, i.unsigned_abs());
    }
}

fn benchmark_unsigned_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.unsigned_abs()", T::NAME),
        BenchmarkType::Single,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|i| no_out!(i.unsigned_abs())))],
    );
}

fn benchmark_abs_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.abs_assign()", T::NAME),
        BenchmarkType::Single,
        signeds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [("malachite", &mut (|mut i| i.abs_assign()))],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $demo_abs_assign_name:ident,
        $demo_unsigned_abs_name:ident,
        $bench_abs_assign_name:ident,
        $bench_unsigned_abs_name:ident
    ) => {
        fn $demo_abs_assign_name(gm: GenerationMode, limit: usize) {
            demo_abs_assign::<$t>(gm, limit);
        }

        fn $demo_unsigned_abs_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_abs::<$t>(gm, limit);
        }

        fn $bench_abs_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_abs_assign::<$t>(gm, limit, file_name);
        }

        fn $bench_unsigned_abs_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_abs::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    i8,
    demo_i8_abs_assign,
    demo_i8_unsigned_abs,
    benchmark_i8_abs_assign,
    benchmark_i8_unsigned_abs
);
demo_and_bench!(
    i16,
    demo_i16_abs_assign,
    demo_i16_unsigned_abs,
    benchmark_i16_abs_assign,
    benchmark_i16_unsigned_abs
);
demo_and_bench!(
    i32,
    demo_i32_abs_assign,
    demo_i32_unsigned_abs,
    benchmark_i32_abs_assign,
    benchmark_i32_unsigned_abs
);
demo_and_bench!(
    i64,
    demo_i64_abs_assign,
    demo_i64_unsigned_abs,
    benchmark_i64_abs_assign,
    benchmark_i64_unsigned_abs
);
demo_and_bench!(
    isize,
    demo_isize_abs_assign,
    demo_isize_unsigned_abs,
    benchmark_isize_abs_assign,
    benchmark_isize_unsigned_abs
);
