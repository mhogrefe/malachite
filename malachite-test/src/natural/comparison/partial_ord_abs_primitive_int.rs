use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::Natural;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_signed, pairs_of_natural_and_unsigned, pairs_of_signed_and_natural,
    pairs_of_unsigned_and_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_partial_cmp_abs_u8);
    register_demo!(registry, demo_natural_partial_cmp_abs_u16);
    register_demo!(registry, demo_natural_partial_cmp_abs_u32);
    register_demo!(registry, demo_natural_partial_cmp_abs_u64);
    register_demo!(registry, demo_natural_partial_cmp_abs_usize);
    register_demo!(registry, demo_u8_partial_cmp_abs_natural);
    register_demo!(registry, demo_u16_partial_cmp_abs_natural);
    register_demo!(registry, demo_u32_partial_cmp_abs_natural);
    register_demo!(registry, demo_u64_partial_cmp_abs_natural);
    register_demo!(registry, demo_usize_partial_cmp_abs_natural);
    register_demo!(registry, demo_natural_partial_cmp_abs_i8);
    register_demo!(registry, demo_natural_partial_cmp_abs_i16);
    register_demo!(registry, demo_natural_partial_cmp_abs_i32);
    register_demo!(registry, demo_natural_partial_cmp_abs_i64);
    register_demo!(registry, demo_natural_partial_cmp_abs_isize);
    register_demo!(registry, demo_i8_partial_cmp_abs_natural);
    register_demo!(registry, demo_i16_partial_cmp_abs_natural);
    register_demo!(registry, demo_i32_partial_cmp_abs_natural);
    register_demo!(registry, demo_i64_partial_cmp_abs_natural);
    register_demo!(registry, demo_isize_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_u8);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_u16);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_u32);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_u64);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_usize);
    register_bench!(registry, Large, benchmark_u8_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_u16_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_u32_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_u64_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_usize_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_i8);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_i16);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_i32);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_i64);
    register_bench!(registry, Large, benchmark_natural_partial_cmp_abs_isize);
    register_bench!(registry, Large, benchmark_i8_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_i16_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_i32_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_i64_partial_cmp_abs_natural);
    register_bench!(registry, Large, benchmark_isize_partial_cmp_abs_natural);
}

fn demo_natural_partial_cmp_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    Natural: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_natural_and_unsigned::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_unsigned_partial_cmp_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_natural::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_natural_partial_cmp_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_natural_and_signed::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_signed_partial_cmp_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_natural::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn benchmark_natural_partial_cmp_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
{
    run_benchmark_old(
        &format!("Natural == {}", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_unsigned_partial_cmp_abs_natural<
    T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{} == Natural", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_natural_partial_cmp_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("Natural == {}", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_signed_partial_cmp_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{} == Natural", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

macro_rules! demo_and_bench {
    (
        $u:ident,
        $s:ident,
        $natural_partial_cmp_unsigned_demo_name:ident,
        $unsigned_partial_cmp_natural_demo_name:ident,
        $natural_partial_cmp_signed_demo_name:ident,
        $signed_partial_cmp_natural_demo_name:ident,
        $natural_partial_cmp_unsigned_bench_name:ident,
        $unsigned_partial_cmp_natural_bench_name:ident,
        $natural_partial_cmp_signed_bench_name:ident,
        $signed_partial_cmp_natural_bench_name:ident
    ) => {
        fn $natural_partial_cmp_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_partial_cmp_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_partial_cmp_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_partial_cmp_abs_natural::<$u>(gm, limit);
        }

        fn $natural_partial_cmp_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_partial_cmp_abs_signed::<$s>(gm, limit);
        }

        fn $signed_partial_cmp_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_partial_cmp_abs_natural::<$s>(gm, limit);
        }

        fn $natural_partial_cmp_unsigned_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_natural_partial_cmp_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_partial_cmp_natural_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_unsigned_partial_cmp_abs_natural::<$u>(gm, limit, file_name);
        }

        fn $natural_partial_cmp_signed_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_natural_partial_cmp_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_partial_cmp_natural_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_signed_partial_cmp_abs_natural::<$s>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    i8,
    demo_natural_partial_cmp_abs_u8,
    demo_u8_partial_cmp_abs_natural,
    demo_natural_partial_cmp_abs_i8,
    demo_i8_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_u8,
    benchmark_u8_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_i8,
    benchmark_i8_partial_cmp_abs_natural
);
demo_and_bench!(
    u16,
    i16,
    demo_natural_partial_cmp_abs_u16,
    demo_u16_partial_cmp_abs_natural,
    demo_natural_partial_cmp_abs_i16,
    demo_i16_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_u16,
    benchmark_u16_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_i16,
    benchmark_i16_partial_cmp_abs_natural
);
demo_and_bench!(
    u32,
    i32,
    demo_natural_partial_cmp_abs_u32,
    demo_u32_partial_cmp_abs_natural,
    demo_natural_partial_cmp_abs_i32,
    demo_i32_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_u32,
    benchmark_u32_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_i32,
    benchmark_i32_partial_cmp_abs_natural
);
demo_and_bench!(
    u64,
    i64,
    demo_natural_partial_cmp_abs_u64,
    demo_u64_partial_cmp_abs_natural,
    demo_natural_partial_cmp_abs_i64,
    demo_i64_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_u64,
    benchmark_u64_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_i64,
    benchmark_i64_partial_cmp_abs_natural
);
demo_and_bench!(
    usize,
    isize,
    demo_natural_partial_cmp_abs_usize,
    demo_usize_partial_cmp_abs_natural,
    demo_natural_partial_cmp_abs_isize,
    demo_isize_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_usize,
    benchmark_usize_partial_cmp_abs_natural,
    benchmark_natural_partial_cmp_abs_isize,
    benchmark_isize_partial_cmp_abs_natural
);
