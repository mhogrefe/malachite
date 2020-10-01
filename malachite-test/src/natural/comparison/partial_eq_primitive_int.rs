use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::comparison::partial_eq_primitive_int::*;
use num::BigUint;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_signed, pairs_of_natural_and_unsigned,
    pairs_of_signed_and_natural, pairs_of_unsigned_and_natural, rm_pairs_of_natural_and_signed,
    rm_pairs_of_signed_and_natural, rm_pairs_of_unsigned_and_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_partial_eq_u8);
    register_demo!(registry, demo_natural_partial_eq_u16);
    register_demo!(registry, demo_natural_partial_eq_u32);
    register_demo!(registry, demo_natural_partial_eq_u64);
    register_demo!(registry, demo_natural_partial_eq_usize);
    register_demo!(registry, demo_u8_partial_eq_natural);
    register_demo!(registry, demo_u16_partial_eq_natural);
    register_demo!(registry, demo_u32_partial_eq_natural);
    register_demo!(registry, demo_u64_partial_eq_natural);
    register_demo!(registry, demo_usize_partial_eq_natural);
    register_demo!(registry, demo_natural_partial_eq_i8);
    register_demo!(registry, demo_natural_partial_eq_i16);
    register_demo!(registry, demo_natural_partial_eq_i32);
    register_demo!(registry, demo_natural_partial_eq_i64);
    register_demo!(registry, demo_natural_partial_eq_isize);
    register_demo!(registry, demo_i8_partial_eq_natural);
    register_demo!(registry, demo_i16_partial_eq_natural);
    register_demo!(registry, demo_i32_partial_eq_natural);
    register_demo!(registry, demo_i64_partial_eq_natural);
    register_demo!(registry, demo_isize_partial_eq_natural);
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_u8_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_u16_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_u64_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_usize_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u8_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u16_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_usize_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_i8_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_i16_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_i64_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_isize_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i8_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i16_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_isize_partial_eq_natural_library_comparison
    );
}

fn demo_natural_partial_eq_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialEq<T>,
{
    for (n, u) in pairs_of_natural_and_unsigned::<T>(gm).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

fn demo_unsigned_partial_eq_natural<T: PartialEq<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (u, n) in pairs_of_unsigned_and_natural::<T>(gm).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

fn demo_natural_partial_eq_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialEq<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, u) in pairs_of_natural_and_signed::<T>(gm).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

fn demo_signed_partial_eq_natural<T: PartialEq<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (u, n) in pairs_of_signed_and_natural::<T>(gm).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

fn benchmark_natural_partial_eq_unsigned_library_comparison<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialEq<T>,
    rug::Integer: PartialEq<T>,
    BigUint: From<T>,
{
    run_benchmark_old(
        &format!("Natural == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x == y))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_eq_unsigned(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x == y))),
        ],
    );
}

fn benchmark_unsigned_partial_eq_natural_library_comparison<
    T: PartialEq<Natural> + PartialEq<rug::Integer> + PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{} == Natural", T::NAME),
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}

fn benchmark_natural_partial_eq_signed_library_comparison<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialEq<T>,
    rug::Integer: PartialEq<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("Natural == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}

fn benchmark_signed_partial_eq_natural_library_comparison<
    T: PartialEq<Natural> + PartialEq<rug::Integer> + PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{} == Natural", T::NAME),
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}

macro_rules! demo_and_bench {
    (
        $u:ident,
        $s:ident,
        $natural_eq_unsigned_demo_name:ident,
        $unsigned_eq_natural_demo_name:ident,
        $natural_eq_signed_demo_name:ident,
        $signed_eq_natural_demo_name:ident,
        $natural_eq_unsigned_bench_name:ident,
        $unsigned_eq_natural_bench_name:ident,
        $natural_eq_signed_bench_name:ident,
        $signed_eq_natural_bench_name:ident
    ) => {
        fn $natural_eq_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_partial_eq_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_eq_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_partial_eq_natural::<$u>(gm, limit);
        }

        fn $natural_eq_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_partial_eq_signed::<$s>(gm, limit);
        }

        fn $signed_eq_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_partial_eq_natural::<$s>(gm, limit);
        }

        fn $natural_eq_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_partial_eq_unsigned_library_comparison::<$u>(gm, limit, file_name);
        }

        fn $unsigned_eq_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_partial_eq_natural_library_comparison::<$u>(gm, limit, file_name);
        }

        fn $natural_eq_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_partial_eq_signed_library_comparison::<$s>(gm, limit, file_name);
        }

        fn $signed_eq_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_partial_eq_natural_library_comparison::<$s>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    i8,
    demo_natural_partial_eq_u8,
    demo_u8_partial_eq_natural,
    demo_natural_partial_eq_i8,
    demo_i8_partial_eq_natural,
    benchmark_natural_partial_eq_u8_library_comparison,
    benchmark_u8_partial_eq_natural_library_comparison,
    benchmark_natural_partial_eq_i8_library_comparison,
    benchmark_i8_partial_eq_natural_library_comparison
);
demo_and_bench!(
    u16,
    i16,
    demo_natural_partial_eq_u16,
    demo_u16_partial_eq_natural,
    demo_natural_partial_eq_i16,
    demo_i16_partial_eq_natural,
    benchmark_natural_partial_eq_u16_library_comparison,
    benchmark_u16_partial_eq_natural_library_comparison,
    benchmark_natural_partial_eq_i16_library_comparison,
    benchmark_i16_partial_eq_natural_library_comparison
);
demo_and_bench!(
    u32,
    i32,
    demo_natural_partial_eq_u32,
    demo_u32_partial_eq_natural,
    demo_natural_partial_eq_i32,
    demo_i32_partial_eq_natural,
    benchmark_natural_partial_eq_u32_library_comparison,
    benchmark_u32_partial_eq_natural_library_comparison,
    benchmark_natural_partial_eq_i32_library_comparison,
    benchmark_i32_partial_eq_natural_library_comparison
);
demo_and_bench!(
    u64,
    i64,
    demo_natural_partial_eq_u64,
    demo_u64_partial_eq_natural,
    demo_natural_partial_eq_i64,
    demo_i64_partial_eq_natural,
    benchmark_natural_partial_eq_u64_library_comparison,
    benchmark_u64_partial_eq_natural_library_comparison,
    benchmark_natural_partial_eq_i64_library_comparison,
    benchmark_i64_partial_eq_natural_library_comparison
);
demo_and_bench!(
    usize,
    isize,
    demo_natural_partial_eq_usize,
    demo_usize_partial_eq_natural,
    demo_natural_partial_eq_isize,
    demo_isize_partial_eq_natural,
    benchmark_natural_partial_eq_usize_library_comparison,
    benchmark_usize_partial_eq_natural_library_comparison,
    benchmark_natural_partial_eq_isize_library_comparison,
    benchmark_isize_partial_eq_natural_library_comparison
);
