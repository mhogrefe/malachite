use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, pairs_of_integer_and_unsigned, pairs_of_signed_and_integer,
    pairs_of_unsigned_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_cmp_abs_u8);
    register_demo!(registry, demo_integer_partial_cmp_abs_u16);
    register_demo!(registry, demo_integer_partial_cmp_abs_u32);
    register_demo!(registry, demo_integer_partial_cmp_abs_u64);
    register_demo!(registry, demo_integer_partial_cmp_abs_usize);
    register_demo!(registry, demo_u8_partial_cmp_abs_integer);
    register_demo!(registry, demo_u16_partial_cmp_abs_integer);
    register_demo!(registry, demo_u32_partial_cmp_abs_integer);
    register_demo!(registry, demo_u64_partial_cmp_abs_integer);
    register_demo!(registry, demo_usize_partial_cmp_abs_integer);
    register_demo!(registry, demo_integer_partial_cmp_abs_i8);
    register_demo!(registry, demo_integer_partial_cmp_abs_i16);
    register_demo!(registry, demo_integer_partial_cmp_abs_i32);
    register_demo!(registry, demo_integer_partial_cmp_abs_i64);
    register_demo!(registry, demo_integer_partial_cmp_abs_isize);
    register_demo!(registry, demo_i8_partial_cmp_abs_integer);
    register_demo!(registry, demo_i16_partial_cmp_abs_integer);
    register_demo!(registry, demo_i32_partial_cmp_abs_integer);
    register_demo!(registry, demo_i64_partial_cmp_abs_integer);
    register_demo!(registry, demo_isize_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_u8);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_u16);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_u32);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_u64);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_usize);
    register_bench!(registry, Large, benchmark_u8_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_u16_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_u32_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_u64_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_usize_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_i8);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_i16);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_i32);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_i64);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_isize);
    register_bench!(registry, Large, benchmark_i8_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_i16_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_i32_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_i64_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_isize_partial_cmp_abs_integer);
}

fn demo_integer_partial_cmp_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    Integer: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_integer_and_unsigned::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_unsigned_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_integer::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_integer_partial_cmp_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_integer_and_signed::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_signed_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_integer::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn benchmark_integer_partial_cmp_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    m_run_benchmark(
        &format!("Integer == {}", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_unsigned_partial_cmp_abs_integer<
    T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{} == Integer", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_integer_partial_cmp_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("Integer == {}", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_signed_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{} == Integer", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

macro_rules! demo_and_bench {
    (
        $u:ident,
        $s:ident,
        $integer_partial_cmp_unsigned_demo_name:ident,
        $unsigned_partial_cmp_integer_demo_name:ident,
        $integer_partial_cmp_signed_demo_name:ident,
        $signed_partial_cmp_integer_demo_name:ident,
        $integer_partial_cmp_unsigned_bench_name:ident,
        $unsigned_partial_cmp_integer_bench_name:ident,
        $integer_partial_cmp_signed_bench_name:ident,
        $signed_partial_cmp_integer_bench_name:ident
    ) => {
        fn $integer_partial_cmp_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_partial_cmp_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_partial_cmp_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_partial_cmp_abs_integer::<$u>(gm, limit);
        }

        fn $integer_partial_cmp_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_partial_cmp_abs_signed::<$s>(gm, limit);
        }

        fn $signed_partial_cmp_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_partial_cmp_abs_integer::<$s>(gm, limit);
        }

        fn $integer_partial_cmp_unsigned_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_integer_partial_cmp_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_partial_cmp_integer_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_unsigned_partial_cmp_abs_integer::<$u>(gm, limit, file_name);
        }

        fn $integer_partial_cmp_signed_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_integer_partial_cmp_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_partial_cmp_integer_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_signed_partial_cmp_abs_integer::<$s>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    i8,
    demo_integer_partial_cmp_abs_u8,
    demo_u8_partial_cmp_abs_integer,
    demo_integer_partial_cmp_abs_i8,
    demo_i8_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_u8,
    benchmark_u8_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_i8,
    benchmark_i8_partial_cmp_abs_integer
);
demo_and_bench!(
    u16,
    i16,
    demo_integer_partial_cmp_abs_u16,
    demo_u16_partial_cmp_abs_integer,
    demo_integer_partial_cmp_abs_i16,
    demo_i16_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_u16,
    benchmark_u16_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_i16,
    benchmark_i16_partial_cmp_abs_integer
);
demo_and_bench!(
    u32,
    i32,
    demo_integer_partial_cmp_abs_u32,
    demo_u32_partial_cmp_abs_integer,
    demo_integer_partial_cmp_abs_i32,
    demo_i32_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_u32,
    benchmark_u32_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_i32,
    benchmark_i32_partial_cmp_abs_integer
);
demo_and_bench!(
    u64,
    i64,
    demo_integer_partial_cmp_abs_u64,
    demo_u64_partial_cmp_abs_integer,
    demo_integer_partial_cmp_abs_i64,
    demo_i64_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_u64,
    benchmark_u64_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_i64,
    benchmark_i64_partial_cmp_abs_integer
);
demo_and_bench!(
    usize,
    isize,
    demo_integer_partial_cmp_abs_usize,
    demo_usize_partial_cmp_abs_integer,
    demo_integer_partial_cmp_abs_isize,
    demo_isize_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_usize,
    benchmark_usize_partial_cmp_abs_integer,
    benchmark_integer_partial_cmp_abs_isize,
    benchmark_isize_partial_cmp_abs_integer
);
