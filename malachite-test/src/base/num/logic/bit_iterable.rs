use std::ops::Index;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitIterable;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned, signeds, unsigneds,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_bits);
    register_demo!(registry, demo_u16_bits);
    register_demo!(registry, demo_u32_bits);
    register_demo!(registry, demo_u64_bits);
    register_demo!(registry, demo_usize_bits);
    register_demo!(registry, demo_i8_bits);
    register_demo!(registry, demo_i16_bits);
    register_demo!(registry, demo_i32_bits);
    register_demo!(registry, demo_i64_bits);
    register_demo!(registry, demo_isize_bits);

    register_demo!(registry, demo_u8_bits_rev);
    register_demo!(registry, demo_u16_bits_rev);
    register_demo!(registry, demo_u32_bits_rev);
    register_demo!(registry, demo_u64_bits_rev);
    register_demo!(registry, demo_usize_bits_rev);
    register_demo!(registry, demo_i8_bits_rev);
    register_demo!(registry, demo_i16_bits_rev);
    register_demo!(registry, demo_i32_bits_rev);
    register_demo!(registry, demo_i64_bits_rev);
    register_demo!(registry, demo_isize_bits_rev);

    register_demo!(registry, demo_u8_bits_size_hint);
    register_demo!(registry, demo_u16_bits_size_hint);
    register_demo!(registry, demo_u32_bits_size_hint);
    register_demo!(registry, demo_u64_bits_size_hint);
    register_demo!(registry, demo_usize_bits_size_hint);

    register_demo!(registry, demo_i8_bits_index);
    register_demo!(registry, demo_i16_bits_index);
    register_demo!(registry, demo_i32_bits_index);
    register_demo!(registry, demo_i64_bits_index);
    register_demo!(registry, demo_isize_bits_index);

    register_bench!(registry, None, benchmark_u8_bits_size_hint);
    register_bench!(registry, None, benchmark_u16_bits_size_hint);
    register_bench!(registry, None, benchmark_u32_bits_size_hint);
    register_bench!(registry, None, benchmark_u64_bits_size_hint);
    register_bench!(registry, None, benchmark_usize_bits_size_hint);

    register_bench!(registry, None, benchmark_u8_bits_get_algorithms);
    register_bench!(registry, None, benchmark_u16_bits_get_algorithms);
    register_bench!(registry, None, benchmark_u32_bits_get_algorithms);
    register_bench!(registry, None, benchmark_u64_bits_get_algorithms);
    register_bench!(registry, None, benchmark_usize_bits_get_algorithms);
    register_bench!(registry, None, benchmark_i8_bits_get_algorithms);
    register_bench!(registry, None, benchmark_i16_bits_get_algorithms);
    register_bench!(registry, None, benchmark_i32_bits_get_algorithms);
    register_bench!(registry, None, benchmark_i64_bits_get_algorithms);
    register_bench!(registry, None, benchmark_isize_bits_get_algorithms);
}

fn demo_unsigned_bits<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!("bits({}) = {:?}", u, u.bits().collect::<Vec<bool>>());
    }
}

fn demo_signed_bits<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!("bits({}) = {:?}", i, i.bits().collect::<Vec<bool>>());
    }
}

fn demo_unsigned_bits_rev<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!(
            "bits({}).rev() = {:?}",
            u,
            u.bits().rev().collect::<Vec<bool>>()
        );
    }
}

fn demo_signed_bits_rev<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for i in signeds::<T>(gm).take(limit) {
        println!(
            "bits({}).rev() = {:?}",
            i,
            i.bits().rev().collect::<Vec<bool>>()
        );
    }
}

fn demo_unsigned_bits_size_hint<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<T>(gm).take(limit) {
        println!("bits({}).size_hint() = {:?}", u, u.bits().size_hint());
    }
}

fn demo_signed_bits_index<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as BitIterable>::BitIterator: Index<u64, Output = bool>,
{
    for (n, i) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!("bits({})[{}] = {:?}", n, i, n.bits()[i]);
    }
}

fn benchmark_unsigned_bits_size_hint<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.bits().size_hint()", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            &format!("{}.bits().size_hint()", T::NAME),
            &mut (|n| no_out!(n.bits().size_hint())),
        )],
    );
}

fn benchmark_unsigned_bits_get_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    <T as BitIterable>::BitIterator: Index<u64, Output = bool>,
{
    run_benchmark_old(
        &format!("{}.bits()[u64]", T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                &format!("{}.bits()[u]", T::NAME),
                &mut (|(n, u)| no_out!(n.bits()[u])),
            ),
            (
                &format!("{}.to_bits_asc()[u]", T::NAME),
                &mut (|(n, u)| {
                    let bits = n.to_bits_asc();
                    let u = usize::exact_from(u);
                    if u >= bits.len() {
                        n < T::ZERO
                    } else {
                        bits[u]
                    };
                }),
            ),
        ],
    );
}

fn benchmark_signed_bits_get_algorithms<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as BitIterable>::BitIterator: Index<u64, Output = bool>,
{
    run_benchmark_old(
        &format!("{}.bits()[u64]", T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                &format!("{}.bits()[u]", T::NAME),
                &mut (|(n, u)| no_out!(n.bits()[u])),
            ),
            (
                &format!("{}.to_bits_asc()[u]", T::NAME),
                &mut (|(n, u)| {
                    let bits = n.to_bits_asc();
                    let u = usize::exact_from(u);
                    if u >= bits.len() {
                        n < T::ZERO
                    } else {
                        bits[u]
                    };
                }),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $bits_demo_name:ident,
        $bits_rev_demo_name:ident,
        $bits_size_hint_demo_name:ident,
        $bits_size_hint_bench_name:ident,
        $bits_get_bench_name:ident
    ) => {
        fn $bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_bits::<$t>(gm, limit);
        }

        fn $bits_rev_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_bits_rev::<$t>(gm, limit);
        }

        fn $bits_size_hint_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_bits_size_hint::<$t>(gm, limit);
        }

        fn $bits_size_hint_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_bits_size_hint::<$t>(gm, limit, file_name);
        }

        fn $bits_get_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_bits_get_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $bits_demo_name:ident,
        $bits_rev_demo_name:ident,
        $bits_index_demo_name:ident,
        $bits_get_bench_name:ident
    ) => {
        fn $bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_bits::<$t>(gm, limit);
        }

        fn $bits_rev_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_bits_rev::<$t>(gm, limit);
        }

        fn $bits_index_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_bits_index::<$t>(gm, limit);
        }

        fn $bits_get_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_bits_get_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_bits,
    demo_u8_bits_rev,
    demo_u8_bits_size_hint,
    benchmark_u8_bits_size_hint,
    benchmark_u8_bits_get_algorithms
);
unsigned!(
    u16,
    demo_u16_bits,
    demo_u16_bits_rev,
    demo_u16_bits_size_hint,
    benchmark_u16_bits_size_hint,
    benchmark_u16_bits_get_algorithms
);
unsigned!(
    u32,
    demo_u32_bits,
    demo_u32_bits_rev,
    demo_u32_bits_size_hint,
    benchmark_u32_bits_size_hint,
    benchmark_u32_bits_get_algorithms
);
unsigned!(
    u64,
    demo_u64_bits,
    demo_u64_bits_rev,
    demo_u64_bits_size_hint,
    benchmark_u64_bits_size_hint,
    benchmark_u64_bits_get_algorithms
);
unsigned!(
    usize,
    demo_usize_bits,
    demo_usize_bits_rev,
    demo_usize_bits_size_hint,
    benchmark_usize_bits_size_hint,
    benchmark_usize_bits_get_algorithms
);
signed!(
    i8,
    demo_i8_bits,
    demo_i8_bits_rev,
    demo_i8_bits_index,
    benchmark_i8_bits_get_algorithms
);
signed!(
    i16,
    demo_i16_bits,
    demo_i16_bits_rev,
    demo_i16_bits_index,
    benchmark_i16_bits_get_algorithms
);
signed!(
    i32,
    demo_i32_bits,
    demo_i32_bits_rev,
    demo_i32_bits_index,
    benchmark_i32_bits_get_algorithms
);
signed!(
    i64,
    demo_i64_bits,
    demo_i64_bits_rev,
    demo_i64_bits_index,
    benchmark_i64_bits_get_algorithms
);
signed!(
    isize,
    demo_isize_bits,
    demo_isize_bits_rev,
    demo_isize_bits_index,
    benchmark_isize_bits_get_algorithms
);
