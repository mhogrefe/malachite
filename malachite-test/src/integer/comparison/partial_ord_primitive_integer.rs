use std::cmp::Ordering;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::comparison::partial_ord_primitive_integer::*;
use num::BigInt;
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{
    nrm_pairs_of_integer_and_signed, nrm_pairs_of_integer_and_unsigned,
    pairs_of_integer_and_signed, pairs_of_integer_and_unsigned, pairs_of_signed_and_integer,
    pairs_of_unsigned_and_integer, rm_pairs_of_signed_and_integer,
    rm_pairs_of_unsigned_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_cmp_u8);
    register_demo!(registry, demo_integer_partial_cmp_u16);
    register_demo!(registry, demo_integer_partial_cmp_u32);
    register_demo!(registry, demo_integer_partial_cmp_u64);
    register_demo!(registry, demo_integer_partial_cmp_usize);
    register_demo!(registry, demo_u8_partial_cmp_integer);
    register_demo!(registry, demo_u16_partial_cmp_integer);
    register_demo!(registry, demo_u32_partial_cmp_integer);
    register_demo!(registry, demo_u64_partial_cmp_integer);
    register_demo!(registry, demo_usize_partial_cmp_integer);
    register_demo!(registry, demo_integer_partial_cmp_i8);
    register_demo!(registry, demo_integer_partial_cmp_i16);
    register_demo!(registry, demo_integer_partial_cmp_i32);
    register_demo!(registry, demo_integer_partial_cmp_i64);
    register_demo!(registry, demo_integer_partial_cmp_isize);
    register_demo!(registry, demo_i8_partial_cmp_integer);
    register_demo!(registry, demo_i16_partial_cmp_integer);
    register_demo!(registry, demo_i32_partial_cmp_integer);
    register_demo!(registry, demo_i64_partial_cmp_integer);
    register_demo!(registry, demo_isize_partial_cmp_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_u8_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_u16_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_u64_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_usize_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u8_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u16_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_usize_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_i8_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_i16_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_i64_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_isize_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i8_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i16_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_partial_cmp_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_isize_partial_cmp_integer_library_comparison
    );
}

fn demo_integer_partial_cmp_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrd<T>,
{
    for (n, u) in pairs_of_integer_and_unsigned::<T>(gm).take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

fn demo_unsigned_partial_cmp_integer<T: PartialOrd<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (u, n) in pairs_of_unsigned_and_integer::<T>(gm).take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

fn demo_integer_partial_cmp_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrd<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, u) in pairs_of_integer_and_signed::<T>(gm).take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

fn demo_signed_partial_cmp_integer<T: PartialOrd<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (u, n) in pairs_of_signed_and_integer::<T>(gm).take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

fn benchmark_integer_partial_cmp_unsigned_library_comparison<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
    BigInt: From<T>,
{
    m_run_benchmark(
        &format!("Integer == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.partial_cmp(&y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_cmp_primitive(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

fn benchmark_unsigned_partial_cmp_integer_library_comparison<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{} == Integer", T::NAME),
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.partial_cmp(&y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

fn benchmark_integer_partial_cmp_signed_library_comparison<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    BigInt: From<T>,
    Integer: PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("Integer == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.partial_cmp(&y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_cmp_primitive(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

fn benchmark_signed_partial_cmp_integer_library_comparison<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{} == Integer", T::NAME),
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.partial_cmp(&y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

macro_rules! demo_and_bench {
    (
        $u:ident,
        $s:ident,
        $integer_eq_unsigned_demo_name:ident,
        $unsigned_eq_integer_demo_name:ident,
        $integer_eq_signed_demo_name:ident,
        $signed_eq_integer_demo_name:ident,
        $integer_eq_unsigned_bench_name:ident,
        $unsigned_eq_integer_bench_name:ident,
        $integer_eq_signed_bench_name:ident,
        $signed_eq_integer_bench_name:ident
    ) => {
        fn $integer_eq_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_partial_cmp_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_eq_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_partial_cmp_integer::<$u>(gm, limit);
        }

        fn $integer_eq_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_partial_cmp_signed::<$s>(gm, limit);
        }

        fn $signed_eq_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_partial_cmp_integer::<$s>(gm, limit);
        }

        fn $integer_eq_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_partial_cmp_unsigned_library_comparison::<$u>(gm, limit, file_name);
        }

        fn $unsigned_eq_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_partial_cmp_integer_library_comparison::<$u>(gm, limit, file_name);
        }

        fn $integer_eq_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_partial_cmp_signed_library_comparison::<$s>(gm, limit, file_name);
        }

        fn $signed_eq_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_partial_cmp_integer_library_comparison::<$s>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    i8,
    demo_integer_partial_cmp_u8,
    demo_u8_partial_cmp_integer,
    demo_integer_partial_cmp_i8,
    demo_i8_partial_cmp_integer,
    benchmark_integer_partial_cmp_u8_library_comparison,
    benchmark_u8_partial_cmp_integer_library_comparison,
    benchmark_integer_partial_cmp_i8_library_comparison,
    benchmark_i8_partial_cmp_integer_library_comparison
);
demo_and_bench!(
    u16,
    i16,
    demo_integer_partial_cmp_u16,
    demo_u16_partial_cmp_integer,
    demo_integer_partial_cmp_i16,
    demo_i16_partial_cmp_integer,
    benchmark_integer_partial_cmp_u16_library_comparison,
    benchmark_u16_partial_cmp_integer_library_comparison,
    benchmark_integer_partial_cmp_i16_library_comparison,
    benchmark_i16_partial_cmp_integer_library_comparison
);
demo_and_bench!(
    u32,
    i32,
    demo_integer_partial_cmp_u32,
    demo_u32_partial_cmp_integer,
    demo_integer_partial_cmp_i32,
    demo_i32_partial_cmp_integer,
    benchmark_integer_partial_cmp_u32_library_comparison,
    benchmark_u32_partial_cmp_integer_library_comparison,
    benchmark_integer_partial_cmp_i32_library_comparison,
    benchmark_i32_partial_cmp_integer_library_comparison
);
demo_and_bench!(
    u64,
    i64,
    demo_integer_partial_cmp_u64,
    demo_u64_partial_cmp_integer,
    demo_integer_partial_cmp_i64,
    demo_i64_partial_cmp_integer,
    benchmark_integer_partial_cmp_u64_library_comparison,
    benchmark_u64_partial_cmp_integer_library_comparison,
    benchmark_integer_partial_cmp_i64_library_comparison,
    benchmark_i64_partial_cmp_integer_library_comparison
);
demo_and_bench!(
    usize,
    isize,
    demo_integer_partial_cmp_usize,
    demo_usize_partial_cmp_integer,
    demo_integer_partial_cmp_isize,
    demo_isize_partial_cmp_integer,
    benchmark_integer_partial_cmp_usize_library_comparison,
    benchmark_usize_partial_cmp_integer_library_comparison,
    benchmark_integer_partial_cmp_isize_library_comparison,
    benchmark_isize_partial_cmp_integer_library_comparison
);
