use std::cmp::max;
use std::fmt::Display;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::integers::_get_bits_naive;
use malachite_base::num::logic::traits::BitBlockAccess;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
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
}

fn demo_unsigned_get_bits<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    <T as BitBlockAccess>::Output: Display,
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
    <T as BitBlockAccess>::Output: Display,
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

fn benchmark_unsigned_get_bits_algorithms<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end)| usize::exact_from(max(start, end))),
        "max(start, end)",
        &mut [
            (
                "default",
                &mut (|(n, start, end)| no_out!(n.get_bits(start, end))),
            ),
            (
                "naive",
                &mut (|(n, start, end)| no_out!(_get_bits_naive::<T, T>(&n, start, end))),
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
    m_run_benchmark(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        triples_of_signed_small_unsigned_and_small_unsigned_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end)| usize::exact_from(max(start, end))),
        "max(start, end)",
        &mut [
            (
                "default",
                &mut (|(n, start, end)| no_out!(n.get_bits(start, end))),
            ),
            (
                "naive",
                &mut (|(n, start, end)| {
                    no_out!(_get_bits_naive::<T, T::UnsignedOfEqualWidth>(
                        &n, start, end
                    ))
                }),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $get_bits_demo_name:ident,
        $get_bits_bench_name:ident
    ) => {
        fn $get_bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_get_bits::<$t>(gm, limit);
        }

        fn $get_bits_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_get_bits_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $get_bits_demo_name:ident,
        $get_bits_bench_name:ident
    ) => {
        fn $get_bits_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_get_bits::<$t>(gm, limit);
        }

        fn $get_bits_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_get_bits_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_get_bits, benchmark_u8_get_bits_algorithms);
unsigned!(u16, demo_u16_get_bits, benchmark_u16_get_bits_algorithms);
unsigned!(u32, demo_u32_get_bits, benchmark_u32_get_bits_algorithms);
unsigned!(u64, demo_u64_get_bits, benchmark_u64_get_bits_algorithms);
unsigned!(
    usize,
    demo_usize_get_bits,
    benchmark_usize_get_bits_algorithms
);
signed!(i8, demo_i8_get_bits, benchmark_i8_get_bits_algorithms);
signed!(i16, demo_i16_get_bits, benchmark_i16_get_bits_algorithms);
signed!(i32, demo_i32_get_bits, benchmark_i32_get_bits_algorithms);
signed!(i64, demo_i64_get_bits, benchmark_i64_get_bits_algorithms);
signed!(
    isize,
    demo_isize_get_bits,
    benchmark_isize_get_bits_algorithms
);
