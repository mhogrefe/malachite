use std::fmt::Display;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::num::logic::bit_block_access::{assign_bits_naive, get_bits_naive};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    quadruples_of_signed_small_u64_small_u64_and_unsigned_var_1,
    quadruples_of_unsigned_small_u64_small_u64_and_unsigned_var_1,
    triples_of_signed_small_unsigned_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_small_unsigned_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_get_bits);
    register_demo!(registry, demo_u16_get_bits);
    register_demo!(registry, demo_u32_get_bits);
    register_demo!(registry, demo_u64_get_bits);
    register_demo!(registry, demo_usize_get_bits);
    register_demo!(registry, demo_i8_get_bits);
    register_demo!(registry, demo_i16_get_bits);
    register_demo!(registry, demo_i32_get_bits);
    register_demo!(registry, demo_i64_get_bits);
    register_demo!(registry, demo_isize_get_bits);

    register_demo!(registry, demo_u8_assign_bits);
    register_demo!(registry, demo_u16_assign_bits);
    register_demo!(registry, demo_u32_assign_bits);
    register_demo!(registry, demo_u64_assign_bits);
    register_demo!(registry, demo_usize_assign_bits);
    register_demo!(registry, demo_i8_assign_bits);
    register_demo!(registry, demo_i16_assign_bits);
    register_demo!(registry, demo_i32_assign_bits);
    register_demo!(registry, demo_i64_assign_bits);
    register_demo!(registry, demo_isize_assign_bits);

    register_bench!(registry, None, benchmark_u8_get_bits_algorithms);
    register_bench!(registry, None, benchmark_u16_get_bits_algorithms);
    register_bench!(registry, None, benchmark_u32_get_bits_algorithms);
    register_bench!(registry, None, benchmark_u64_get_bits_algorithms);
    register_bench!(registry, None, benchmark_usize_get_bits_algorithms);
    register_bench!(registry, None, benchmark_i8_get_bits_algorithms);
    register_bench!(registry, None, benchmark_i16_get_bits_algorithms);
    register_bench!(registry, None, benchmark_i32_get_bits_algorithms);
    register_bench!(registry, None, benchmark_i64_get_bits_algorithms);
    register_bench!(registry, None, benchmark_isize_get_bits_algorithms);

    register_bench!(registry, None, benchmark_u8_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_u16_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_u32_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_u64_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_usize_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_i8_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_i16_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_i32_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_i64_assign_bits_algorithms);
    register_bench!(registry, None, benchmark_isize_assign_bits_algorithms);
}

fn demo_unsigned_get_bits<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    <T as BitBlockAccess>::Bits: Display,
{
    for (n, start, end) in
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_1::<T, u64>(gm).take(limit)
    {
        println!(
            "{}.get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn demo_signed_get_bits<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as BitBlockAccess>::Bits: Display,
{
    for (n, start, end) in
        triples_of_signed_small_unsigned_and_small_unsigned_var_1::<T, u64>(gm).take(limit)
    {
        println!(
            "{}.get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn demo_unsigned_assign_bits<T: Display + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: BitBlockAccess<Bits = T>,
{
    for (mut n, start, end, bits) in
        quadruples_of_unsigned_small_u64_small_u64_and_unsigned_var_1::<T, T>(gm).take(limit)
    {
        let old_n = n;
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n,
        );
    }
}

fn demo_signed_assign_bits<T: PrimitiveSigned + Rand, U: Display + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: BitBlockAccess<Bits = U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
{
    for (mut n, start, end, bits) in
        quadruples_of_signed_small_u64_small_u64_and_unsigned_var_1::<T, U>(gm).take(limit)
    {
        let old_n = n;
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n,
        );
    }
}

fn benchmark_unsigned_get_bits_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end)| usize::exact_from(end - start)),
        "end - start",
        &mut [
            (
                "default",
                &mut (|(n, start, end)| no_out!(n.get_bits(start, end))),
            ),
            (
                "naive",
                &mut (|(n, start, end)| no_out!(get_bits_naive::<T, T>(&n, start, end))),
            ),
        ],
    );
}

fn benchmark_signed_get_bits_algorithms<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        triples_of_signed_small_unsigned_and_small_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end)| usize::exact_from(end - start)),
        "end - start",
        &mut [
            (
                "default",
                &mut (|(n, start, end)| no_out!(n.get_bits(start, end))),
            ),
            (
                "naive",
                &mut (|(n, start, end)| {
                    no_out!(get_bits_naive::<T, T::UnsignedOfEqualWidth>(&n, start, end))
                }),
            ),
        ],
    );
}

fn benchmark_unsigned_assign_bits_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: BitBlockAccess<Bits = T>,
{
    run_benchmark_old(
        &format!("{}.assign_bits(u64, u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        quadruples_of_unsigned_small_u64_small_u64_and_unsigned_var_1::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end, _)| usize::exact_from(end - start)),
        "end - start",
        &mut [
            (
                "default",
                &mut (|(mut n, start, end, bits)| no_out!(n.assign_bits(start, end, &bits))),
            ),
            (
                "naive",
                &mut (|(mut n, start, end, bits)| {
                    no_out!(assign_bits_naive::<T, <T as BitBlockAccess>::Bits>(
                        &mut n, start, end, &bits
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_signed_assign_bits_algorithms<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: BitBlockAccess<Bits = U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
{
    run_benchmark_old(
        &format!("{}.assign_bits(u64, u64, {})", T::NAME, U::NAME),
        BenchmarkType::Algorithms,
        quadruples_of_signed_small_u64_small_u64_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end, _)| usize::exact_from(end - start)),
        "end - start",
        &mut [
            (
                "default",
                &mut (|(mut n, start, end, bits)| no_out!(n.assign_bits(start, end, &bits))),
            ),
            (
                "naive",
                &mut (|(mut n, start, end, bits)| {
                    no_out!(assign_bits_naive::<T, U>(&mut n, start, end, &bits))
                }),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $get_bits_demo_name:ident,
        $get_bits_bench_name:ident,
        $assign_bits_demo_name:ident,
        $assign_bits_bench_name:ident
    ) => {
        fn $get_bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_get_bits::<$t>(gm, limit);
        }

        fn $assign_bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_assign_bits::<$t>(gm, limit);
        }

        fn $get_bits_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_get_bits_algorithms::<$t>(gm, limit, file_name);
        }

        fn $assign_bits_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_assign_bits_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $get_bits_demo_name:ident,
        $get_bits_bench_name:ident,
        $assign_bits_demo_name:ident,
        $assign_bits_bench_name:ident
    ) => {
        fn $get_bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_get_bits::<$t>(gm, limit);
        }

        fn $assign_bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_assign_bits::<$t, <$t as PrimitiveSigned>::UnsignedOfEqualWidth>(gm, limit);
        }

        fn $get_bits_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_get_bits_algorithms::<$t>(gm, limit, file_name);
        }

        fn $assign_bits_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_assign_bits_algorithms::<
                $t,
                <$t as PrimitiveSigned>::UnsignedOfEqualWidth,
            >(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_get_bits,
    benchmark_u8_get_bits_algorithms,
    demo_u8_assign_bits,
    benchmark_u8_assign_bits_algorithms
);
unsigned!(
    u16,
    demo_u16_get_bits,
    benchmark_u16_get_bits_algorithms,
    demo_u16_assign_bits,
    benchmark_u16_assign_bits_algorithms
);
unsigned!(
    u32,
    demo_u32_get_bits,
    benchmark_u32_get_bits_algorithms,
    demo_u32_assign_bits,
    benchmark_u32_assign_bits_algorithms
);
unsigned!(
    u64,
    demo_u64_get_bits,
    benchmark_u64_get_bits_algorithms,
    demo_u64_assign_bits,
    benchmark_u64_assign_bits_algorithms
);
unsigned!(
    usize,
    demo_usize_get_bits,
    benchmark_usize_get_bits_algorithms,
    demo_usize_assign_bits,
    benchmark_usize_assign_bits_algorithms
);

signed!(
    i8,
    demo_i8_get_bits,
    benchmark_i8_get_bits_algorithms,
    demo_i8_assign_bits,
    benchmark_i8_assign_bits_algorithms
);
signed!(
    i16,
    demo_i16_get_bits,
    benchmark_i16_get_bits_algorithms,
    demo_i16_assign_bits,
    benchmark_i16_assign_bits_algorithms
);
signed!(
    i32,
    demo_i32_get_bits,
    benchmark_i32_get_bits_algorithms,
    demo_i32_assign_bits,
    benchmark_i32_assign_bits_algorithms
);
signed!(
    i64,
    demo_i64_get_bits,
    benchmark_i64_get_bits_algorithms,
    demo_i64_assign_bits,
    benchmark_i64_assign_bits_algorithms
);
signed!(
    isize,
    demo_isize_get_bits,
    benchmark_isize_get_bits_algorithms,
    demo_isize_assign_bits,
    benchmark_isize_assign_bits_algorithms
);
